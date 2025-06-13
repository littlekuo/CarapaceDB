use std::{sync::{atomic::{AtomicU64, Ordering}, Arc, Mutex}};
use std::collections::VecDeque;

use crate::{catalog::catalog_set::CatalogSet, storage::storage_manager::StorageManager};

pub struct Transaction; // temp

type TransactionId = u64;

struct StoredCatalogSet {
    stored_set: Box<CatalogSet>,
    highest_active_query: TransactionId,
}

pub struct TransactionManager {
    storage: Arc<StorageManager>,
    
    current_query_number: AtomicU64,
    inner: Mutex<TransactionManagerInner>,
}

struct TransactionManagerInner {
    current_start_timestamp: TransactionId,
    current_transaction_id: TransactionId,
    active_transactions: Vec<Box<Transaction>>,
    recently_committed_transactions: VecDeque<Box<Transaction>>,
    // Transactions awaiting GC
    old_transactions: Vec<Box<Transaction>>,
    old_catalog_sets: Vec<StoredCatalogSet>,
}

impl TransactionManager {
    pub fn new(storage: Arc<StorageManager>) -> Self {
        Self {
            storage,
            current_query_number: AtomicU64::new(0),
            inner: Mutex::new(TransactionManagerInner {
                current_start_timestamp: 0,
                current_transaction_id: 1, // 从1开始
                active_transactions: Vec::new(),
                recently_committed_transactions: VecDeque::new(),
                old_transactions: Vec::new(),
                old_catalog_sets: Vec::new(),
            }),
        }
    }

    /// 启动新事务
    pub fn start_transaction(&self) -> Arc<Transaction> {
        let mut inner = self.inner.lock().unwrap();
        
        // 创建新事务
        let transaction = Arc::new(Transaction::new(
            inner.current_transaction_id,
            inner.current_start_timestamp
        ));
        
        // 更新内部状态
        inner.current_transaction_id += 1;
        inner.current_start_timestamp += 1;
        inner.active_transactions.push(transaction.clone());
        
        transaction
    }

    /// 提交事务
    pub fn commit_transaction(&self, transaction: Arc<Transaction>) {
        let mut inner = self.inner.lock().unwrap();
        
        // 从活动事务中移除
        if let Some(pos) = inner.active_transactions.iter()
            .position(|t| Arc::ptr_eq(t, &transaction)) 
        {
            inner.active_transactions.remove(pos);
        }
        
        // 添加到最近提交列表
        inner.recently_committed_transactions.push_back(transaction);
    }

    pub fn rollback_transaction(&self, transaction: Arc<Transaction>) {
        let mut inner = self.inner.lock().unwrap();
        
        if let Some(pos) = inner.active_transactions.iter()
            .position(|t| Arc::ptr_eq(t, &transaction)) 
        {
            inner.active_transactions.remove(pos);
        }
        
        transaction.rollback();
        
        inner.old_transactions.push(transaction);
    }

    pub fn add_catalog_set(&self, context: &ClientContext, catalog_set: Box<CatalogSet>) {
        let mut inner = self.inner.lock().unwrap();
        
        let query_number = self.current_query_number.load(Ordering::SeqCst);
        
        inner.old_catalog_sets.push(StoredCatalogSet {
            stored_set: catalog_set,
            highest_active_query: query_number,
        });
    }

    pub fn get_query_number(&self) -> TransactionId {
        self.current_query_number.fetch_add(1, Ordering::SeqCst)
    }

    fn remove_transaction(&self, transaction: &Arc<Transaction>) {
        let mut inner = self.inner.lock().unwrap();
        
        if let Some(pos) = inner.active_transactions.iter()
            .position(|t| Arc::ptr_eq(t, transaction)) 
        {
            inner.active_transactions.remove(pos);
        }
    }
}
