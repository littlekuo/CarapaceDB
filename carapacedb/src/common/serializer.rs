use std::io::{self, Read, Write};


pub trait Serializable {
   fn serialize<S: Serializer>(&self, serializer: &mut S) -> io::Result<()>;
}
pub trait Deserializable {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> io::Result<Self> where Self: Sized;
}

pub trait Serializer {
    fn write_data(&mut self, buffer: &[u8]) -> io::Result<()>;
    
    fn write<T: Copy>(&mut self, element: T) -> io::Result<()> {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                &element as *const T as *const u8,
                std::mem::size_of::<T>()
            )
        };
        self.write_data(bytes)
    }
    
    fn write_string(&mut self, val: &str) -> io::Result<()> {
        debug_assert!(val.len() <= u32::MAX as usize);
        self.write::<u32>(val.len() as u32)?;
        if !val.is_empty() {
            self.write_data(val.as_bytes())?;
        }
        Ok(())
    }
    
    fn write_list<T: Serializable>(&mut self, list: &[T]) -> io::Result<()> {
        assert!(list.len() <= u32::MAX as usize);
        self.write::<u32>(list.len() as u32)?;
        for item in list {
            item.serialize(self)?;
        }
        Ok(())
    }
    
    fn write_optional<T: Serializable>(&mut self, element: &Option<T>) -> io::Result<()> {
        self.write::<bool>(element.is_some())?;
        if let Some(item) = element {
            item.serialize(self)?;
        }
        Ok(())
    }
}


pub trait Deserializer {
    fn read_data(&mut self, buffer: &mut [u8]) -> io::Result<()>;
    
    fn read<T: Copy + Default>(&mut self) -> io::Result<T> {
        let mut value = T::default();
        let bytes = unsafe {
            std::slice::from_raw_parts_mut(
                &mut value as *mut T as *mut u8,
                std::mem::size_of::<T>()
            )
        };
        self.read_data(bytes)?;
        Ok(value)
    }
    
    fn read_string(&mut self) -> io::Result<String> {
        let len = self.read::<u32>()? as usize;
        let mut bytes = vec![0u8; len];
        if len > 0 {
            self.read_data(&mut bytes)?;
        }
        String::from_utf8(bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
    
    fn read_list<T: Deserializable>(&mut self) -> io::Result<Vec<T>> {
        let count = self.read::<u32>()? as usize;
        let mut list = Vec::with_capacity(count);
        for _ in 0..count {
            list.push(T::deserialize(self)?);
        }
        Ok(list)
    }
    
    fn read_optional<T: Deserializable>(&mut self) -> io::Result<Option<T>> {
        let has_entry = self.read::<bool>()?;
        if has_entry {
            Ok(Some(T::deserialize(self)?))
        } else {
            Ok(None)
        }
    }
}
