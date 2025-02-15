use std::sync::Mutex;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lazy_static::lazy_static;
use rl_ball_sym::{
    load_dropshot, load_hoops, load_soccar, load_soccar_throwback,
    simulation::{ball::Ball, game::Game},
};

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(load_soccar());
}

// This bench seems unnecssary, it seems to only test how long it takes to lock a mutex.
fn init_benchmark(c: &mut Criterion) {
    c.bench_function("init", |b| {
        b.iter(|| {
            let mut game = GAME.lock().unwrap();
            game.ball.location.z = 1900.;
            Ball::get_ball_prediction_struct(&mut game);
        })
    });
}

fn load_soccar_benchmark(c: &mut Criterion) {
    c.bench_function("load_soccar", |b| b.iter(load_soccar));
}

fn load_hoops_benchmark(c: &mut Criterion) {
    c.bench_function("load_hoops", |b| b.iter(load_hoops));
}

fn load_dropshot_benchmark(c: &mut Criterion) {
    c.bench_function("load_dropshot", |b| b.iter(load_dropshot));
}

fn load_soccar_throwback_benchmark(c: &mut Criterion) {
    c.bench_function("load_soccar_throwback", |b| b.iter(load_soccar_throwback));
}

fn get_ball_prediction_struct_with_time_benchmark(c: &mut Criterion) {
    let mut game = load_soccar();
    let time = 8.;

    c.bench_with_input(BenchmarkId::new("get_ball_prediction_struct_for_time", time), &time, |b, time| b.iter(|| Ball::get_ball_prediction_struct_for_time(black_box(&mut game), time)));
}

fn get_ball_prediction_struct_benchmark(c: &mut Criterion) {
    let mut game = load_soccar();
    c.bench_function("get_ball_prediction/soccar", |b| b.iter(|| Ball::get_ball_prediction_struct(black_box(&mut game))));
}

fn get_ball_prediction_struct_hoops_benchmark(c: &mut Criterion) {
    let mut game = load_hoops();

    c.bench_function("get_ball_prediction/hoops", |b| b.iter(|| Ball::get_ball_prediction_struct(black_box(&mut game))));
}

fn get_ball_prediction_struct_dropshot(c: &mut Criterion) {
    let mut game = load_dropshot();

    c.bench_function("get_ball_prediction/dropshot", |b| b.iter(|| Ball::get_ball_prediction_struct(black_box(&mut game))));
}

fn get_ball_prediction_struct_throwback(c: &mut Criterion) {
    let mut game = load_soccar_throwback();

    c.bench_function("get_ball_prediction/throwback", |b| b.iter(|| Ball::get_ball_prediction_struct(black_box(&mut game))));
}

criterion_group!(init, init_benchmark, load_soccar_benchmark, load_hoops_benchmark, load_dropshot_benchmark, load_soccar_throwback_benchmark,);
criterion_group!(prediction, get_ball_prediction_struct_with_time_benchmark, get_ball_prediction_struct_benchmark, get_ball_prediction_struct_hoops_benchmark, get_ball_prediction_struct_dropshot, get_ball_prediction_struct_throwback);
criterion_main!(init, prediction);
