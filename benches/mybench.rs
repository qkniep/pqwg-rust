use boringtun::noise::tests::tests::wireguard_handshake;
use criterion::{black_box, criterion_group, criterion_main, Criterion, ParameterizedBenchmark};

fn bench_handshake(c: &mut Criterion) {
    c.bench(
        "Level 1",
        ParameterizedBenchmark::new(
            "Handshake",
            |b, param| b.iter(|| {
                black_box(wireguard_handshake(*param));
            }),
            vec![1u32, 5u32, 10u32, 20u32, 40u32, 60u32, 80u32, 100u32],
        )
    );
    //c.bench_function("WireGuard Handshake", |b| b.iter(|| {
    //    black_box(wireguard_handshake());
    //}));
}

criterion_group!(benches, bench_handshake);
criterion_main!(benches);
