#![feature(test)]

extern crate test;

const INPUT: &str = "sin(pi / 2) + cos(0) - tan(45) * 2^3 + log(10, 100) + sqrt(16)";

#[bench]
fn bench_tokenize(b: &mut test::Bencher) {
	let input = test::black_box(INPUT);
	b.iter(|| {
		pupil::tokenize(input).count()
	});
}

#[bench]
fn bench_eval(b: &mut test::Bencher) {
	let env = pupil::BasicEnv::default();
	let input = test::black_box(INPUT);
	b.iter(|| {
		pupil::eval(&env, input)
	});
}

#[bench]
fn bench_eval_tokens(b: &mut test::Bencher) {
	let env = pupil::BasicEnv::default();
	let input = test::black_box(INPUT);
	let tokens: Vec<_> = pupil::tokenize(input).collect();
	b.iter(|| {
		pupil::eval_tokens(&env, &tokens)
	});
}
