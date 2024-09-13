// src/traits/as_str.rs : `AsStr`

/// Trait defining instance method `as_str() : &str` that allows a type to
/// expose its contiguous character representation to client code.
pub trait AsStr {
    fn as_str(&self) -> &str;
}


impl<T : AsStr + ?Sized> AsStr for Box<T> {
    fn as_str(&self) -> &str {
        (**self).as_str()
    }
}

impl<T : AsStr + ?Sized> AsStr for std::rc::Rc<T> {
    fn as_str(&self) -> &str {
        (**self).as_str()
    }
}


#[cfg(feature = "implement-AsStr-for-built_ins")]
mod impl_for_built_ins {

    impl super::AsStr for str {
        fn as_str(&self) -> &str {
            self
        }
    }

    impl super::AsStr for &str {
        fn as_str(&self) -> &str {
            self
        }
    }
}

#[cfg(feature = "implement-AsStr-for-standard_collection_types")]
mod impl_for_std_coll_types {

    impl super::AsStr for String {
        fn as_str(&self) -> &str {
            self.as_str()
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::AsStr;

    use std::rc::Rc;


    #[allow(unused)]
    fn as_AsStr<T : AsStr>(t : &T) -> &impl AsStr {
        t
    }


    mod TEST_CUSTOM_TYPE {
        #![allow(non_snake_case)]

        use super::*;


        #[derive(Debug)]
        struct CustomType {
            s : String,
        }

        impl AsStr for CustomType {
            fn as_str(&self) -> &str {
                &self.s
            }
        }


        #[test]
        fn TEST_AS_VALUE() {
            let ct = CustomType { s : "abc".into() };

            assert_eq!("abc", ct.as_str());

            let ct = &ct;

            assert_eq!("abc", ct.as_str());
        }

        #[test]
        fn TEST_IN_Box() {
            {
                let v = CustomType { s : "abc".into() };
                let ct = Box::new(v);

                assert_eq!("abc", ct.as_str());

                let ct = &ct;

                assert_eq!("abc", ct.as_str());
            }

            {
                let v = CustomType { s : "abc".into() };
                let ct = &Box::new(v);

                assert_eq!("abc", ct.as_str());

                let ct = &ct;

                assert_eq!("abc", ct.as_str());
            }

            {
                let v = CustomType { s : "abc".into() };
                let ct = Box::new(&v);

                assert_eq!("abc", ct.as_str());

                let ct = &ct;

                assert_eq!("abc", ct.as_str());
            }

            {
                let v = CustomType { s : "abc".into() };
                let ct = &Box::new(&v);

                assert_eq!("abc", ct.as_str());

                let ct = &ct;

                assert_eq!("abc", ct.as_str());
            }
        }

        #[test]
        fn TEST_IN_Rc() {
            {
                let v = CustomType { s : "abc".into() };
                let ct = Rc::new(v);

                assert_eq!("abc", ct.as_str());

                let ct = &ct;

                assert_eq!("abc", ct.as_str());
            }

            {
                let v = CustomType { s : "abc".into() };
                let ct = &Rc::new(v);

                assert_eq!("abc", ct.as_str());

                let ct = &ct;

                assert_eq!("abc", ct.as_str());
            }

            {
                let v = CustomType { s : "abc".into() };
                let ct = Rc::new(&v);

                assert_eq!("abc", ct.as_str());

                let ct = &ct;

                assert_eq!("abc", ct.as_str());
            }

            {
                let v = CustomType { s : "abc".into() };
                let ct = &Rc::new(&v);

                assert_eq!("abc", ct.as_str());

                let ct = &ct;

                assert_eq!("abc", ct.as_str());
            }
        }
    }
}

