extern crate flame;
#[macro_use] extern crate flamer;
extern crate im_rc;

pub mod vm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
