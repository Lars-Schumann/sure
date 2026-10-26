macro_rules! for_ {
    ($loop_variable:ident in 0..$upper:ident $body:block) => {{
        let mut $loop_variable: usize = 0;
        while $loop_variable < $upper {
            $body
            $loop_variable += 1;
        }
        #[expect(unused)]
        let $loop_variable: !;
    }};
}
pub(crate) use for_;
