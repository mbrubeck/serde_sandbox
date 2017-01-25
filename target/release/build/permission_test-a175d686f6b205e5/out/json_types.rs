use std::collections::HashSet;
use std::hash::{BuildHasherDefault, Hash};
use seahash::SeaHasher;

type SeaHashSet<K> = HashSet<K, BuildHasherDefault<SeaHasher>>;




#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Point: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for Point {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<Point, __D::Error> where
             __D: _serde::de::Deserializer {
                #[allow(non_camel_case_types)]
                enum __Field { __field0, __field1, __ignore, }
                impl _serde::de::Deserialize for __Field {
                    #[inline]
                    fn deserialize<__D>(deserializer: &mut __D)
                     -> ::std::result::Result<__Field, __D::Error> where
                     __D: _serde::de::Deserializer {
                        struct __FieldVisitor;
                        impl _serde::de::Visitor for __FieldVisitor {
                            type
                            Value
                            =
                            __Field;
                            fn visit_usize<__E>(&mut self, value: usize)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    0usize => { Ok(__Field::__field0) }
                                    1usize => { Ok(__Field::__field1) }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(&mut self, value: &str)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    "x" => { Ok(__Field::__field0) }
                                    "y" => { Ok(__Field::__field1) }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(&mut self, value: &[u8])
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    b"x" => { Ok(__Field::__field0) }
                                    b"y" => { Ok(__Field::__field1) }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                        }
                        deserializer.deserialize_struct_field(__FieldVisitor)
                    }
                }
                struct __Visitor;
                impl _serde::de::Visitor for __Visitor {
                    type
                    Value
                    =
                    Point;
                    #[inline]
                    fn visit_seq<__V>(&mut self, mut visitor: __V)
                     -> ::std::result::Result<Point, __V::Error> where
                     __V: _serde::de::SeqVisitor {
                        let __field0 =
                            match try!(visitor . visit :: < i32 > (  )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(0usize));
                                }
                            };
                        let __field1 =
                            match try!(visitor . visit :: < i32 > (  )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(1usize));
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(Point{x: __field0, y: __field1,})
                    }
                    #[inline]
                    fn visit_map<__V>(&mut self, mut visitor: __V)
                     -> ::std::result::Result<Point, __V::Error> where
                     __V: _serde::de::MapVisitor {
                        let mut __field0: Option<i32> = None;
                        let mut __field1: Option<i32> = None;
                        while let Some(key) =
                                  try!(visitor . visit_key :: < __Field > (
                                       )) {
                            match key {
                                __Field::__field0 => {
                                    if __field0.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("x"));
                                    }
                                    __field0 =
                                        Some(try!(visitor . visit_value :: <
                                                  i32 > (  )));
                                }
                                __Field::__field1 => {
                                    if __field1.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("y"));
                                    }
                                    __field1 =
                                        Some(try!(visitor . visit_value :: <
                                                  i32 > (  )));
                                }
                                _ => {
                                    let _ =
                                        try!(visitor . visit_value :: < _serde
                                             :: de :: impls :: IgnoredAny > (
                                             ));
                                }
                            }
                        }
                        try!(visitor . end (  ));
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None => try!(visitor . missing_field ( "x" )),
                            };
                        let __field1 =
                            match __field1 {
                                Some(__field1) => __field1,
                                None => try!(visitor . missing_field ( "y" )),
                            };
                        Ok(Point{x: __field0, y: __field1,})
                    }
                }
                const FIELDS: &'static [&'static str] = &["x", "y"];
                deserializer.deserialize_struct("Point", FIELDS, __Visitor)
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Point: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for Point {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                let mut __serde_state =
                    try!(_serializer . serialize_struct ( "Point" , 0 + 1 + 1
                         ));
                try!(_serializer . serialize_struct_elt (
                     & mut __serde_state , "x" , & self . x ));
                try!(_serializer . serialize_struct_elt (
                     & mut __serde_state , "y" , & self . y ));
                _serializer.serialize_struct_end(__serde_state)
            }
        }
    };
#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Permission: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl <T: Hash + Eq> _serde::de::Deserialize for Permission<T> where
         T: _serde::de::Deserialize {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<Permission<T>, __D::Error> where
             __D: _serde::de::Deserializer {
                #[allow(non_camel_case_types)]
                enum __Field { __field0, __field1, __field2, __ignore, }
                impl _serde::de::Deserialize for __Field {
                    #[inline]
                    fn deserialize<__D>(deserializer: &mut __D)
                     -> ::std::result::Result<__Field, __D::Error> where
                     __D: _serde::de::Deserializer {
                        struct __FieldVisitor;
                        impl _serde::de::Visitor for __FieldVisitor {
                            type
                            Value
                            =
                            __Field;
                            fn visit_usize<__E>(&mut self, value: usize)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    0usize => { Ok(__Field::__field0) }
                                    1usize => { Ok(__Field::__field1) }
                                    2usize => { Ok(__Field::__field2) }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(&mut self, value: &str)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    "domain" => { Ok(__Field::__field0) }
                                    "actions" => { Ok(__Field::__field1) }
                                    "targets" => { Ok(__Field::__field2) }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(&mut self, value: &[u8])
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    b"domain" => { Ok(__Field::__field0) }
                                    b"actions" => { Ok(__Field::__field1) }
                                    b"targets" => { Ok(__Field::__field2) }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                        }
                        deserializer.deserialize_struct_field(__FieldVisitor)
                    }
                }
                struct __Visitor<T: Hash + Eq>(::std::marker::PhantomData<T>)
                       where T: _serde::de::Deserialize;
                impl <T: Hash + Eq> _serde::de::Visitor for __Visitor<T> where
                 T: _serde::de::Deserialize {
                    type
                    Value
                    =
                    Permission<T>;
                    #[inline]
                    fn visit_seq<__V>(&mut self, mut visitor: __V)
                     -> ::std::result::Result<Permission<T>, __V::Error> where
                     __V: _serde::de::SeqVisitor {
                        let __field0 =
                            match try!(visitor . visit :: < Option < T > > (
                                       )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(0usize));
                                }
                            };
                        let __field1 =
                            match try!(visitor . visit :: < Option <
                                       SeaHashSet < T > > > (  )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(1usize));
                                }
                            };
                        let __field2 =
                            match try!(visitor . visit :: < Option <
                                       SeaHashSet < T > > > (  )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(2usize));
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(Permission{domain: __field0,
                                      actions: __field1,
                                      targets: __field2,})
                    }
                    #[inline]
                    fn visit_map<__V>(&mut self, mut visitor: __V)
                     -> ::std::result::Result<Permission<T>, __V::Error> where
                     __V: _serde::de::MapVisitor {
                        let mut __field0: Option<Option<T>> = None;
                        let mut __field1: Option<Option<SeaHashSet<T>>> =
                            None;
                        let mut __field2: Option<Option<SeaHashSet<T>>> =
                            None;
                        while let Some(key) =
                                  try!(visitor . visit_key :: < __Field > (
                                       )) {
                            match key {
                                __Field::__field0 => {
                                    if __field0.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("domain"));
                                    }
                                    __field0 =
                                        Some(try!(visitor . visit_value :: <
                                                  Option < T > > (  )));
                                }
                                __Field::__field1 => {
                                    if __field1.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("actions"));
                                    }
                                    __field1 =
                                        Some(try!(visitor . visit_value :: <
                                                  Option < SeaHashSet < T > >
                                                  > (  )));
                                }
                                __Field::__field2 => {
                                    if __field2.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("targets"));
                                    }
                                    __field2 =
                                        Some(try!(visitor . visit_value :: <
                                                  Option < SeaHashSet < T > >
                                                  > (  )));
                                }
                                _ => {
                                    let _ =
                                        try!(visitor . visit_value :: < _serde
                                             :: de :: impls :: IgnoredAny > (
                                             ));
                                }
                            }
                        }
                        try!(visitor . end (  ));
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                try!(visitor . missing_field ( "domain" )),
                            };
                        let __field1 =
                            match __field1 {
                                Some(__field1) => __field1,
                                None =>
                                try!(visitor . missing_field ( "actions" )),
                            };
                        let __field2 =
                            match __field2 {
                                Some(__field2) => __field2,
                                None =>
                                try!(visitor . missing_field ( "targets" )),
                            };
                        Ok(Permission{domain: __field0,
                                      actions: __field1,
                                      targets: __field2,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["domain", "actions", "targets"];
                deserializer.deserialize_struct("Permission", FIELDS,
                                                __Visitor::<T>(::std::marker::PhantomData))
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Permission: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl <T: Hash + Eq> _serde::ser::Serialize for Permission<T> where
         T: _serde::ser::Serialize {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                let mut __serde_state =
                    try!(_serializer . serialize_struct (
                         "Permission" , 0 + 1 + 1 + 1 ));
                try!(_serializer . serialize_struct_elt (
                     & mut __serde_state , "domain" , & self . domain ));
                try!(_serializer . serialize_struct_elt (
                     & mut __serde_state , "actions" , & self . actions ));
                try!(_serializer . serialize_struct_elt (
                     & mut __serde_state , "targets" , & self . targets ));
                _serializer.serialize_struct_end(__serde_state)
            }
        }
    };
#[derive(Debug)]
pub struct Permission<T: Hash + Eq> {
    pub domain: Option<T>,
    pub actions: Option<SeaHashSet<T>>,
    pub targets: Option<SeaHashSet<T>>,
}
