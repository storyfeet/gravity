
pub mod ec_vec;
pub use self::ec_vec::*;
pub mod gen;
pub use self::gen::*;



#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_gen_add_drop(){
        // this test works because the GenManager will always add
        // at the lowest available position
        let mut gm = GenManager::new();
        for _ in 0..5 {
            gm.add();
        }
        let gi5 = gm.add();
        for _ in 0..5 {
            gm.add();
        }
        gm.drop_item(gi5);
        let gi11 = gm.add();
        assert_eq!(gi11.loc,5);
        assert_eq!(gi11.gen,11);

        let mut ecv = ECVec::new();
        ecv.put(gi5,20);
        ecv.put(gi11,30);

        assert!(ecv.get(gi5).is_none());
        assert_eq!(ecv.get(gi11),Some(&30));
    }

}
