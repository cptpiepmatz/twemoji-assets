use criterion::{Criterion, black_box, criterion_group, criterion_main};
use twemoji_assets::svg::SvgTwemojiAsset;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("from_name(BUCKET)", |b| b.iter(|| SvgTwemojiAsset::from_name(black_box("bucket"))));
    c.bench_function("from_name(DUCK)", |b| b.iter(|| SvgTwemojiAsset::from_name(black_box("duck"))));
    c.bench_function("from_name(WARNING)", |b| b.iter(|| SvgTwemojiAsset::from_name(black_box("warning"))));
    c.bench_function("from_name(HEART ON FIRE)", |b| b.iter(|| SvgTwemojiAsset::from_name(black_box("heart_on_fire"))));

    c.bench_function("from_name(no emoji)", |b| b.iter(|| SvgTwemojiAsset::from_name(black_box("not an emoji"))));
    c.bench_function("from_name(empty string)", |b| b.iter(|| SvgTwemojiAsset::from_name(black_box(""))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
