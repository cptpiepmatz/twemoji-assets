use criterion::{Criterion, black_box, criterion_group, criterion_main};
use twemoji_assets::svg::SvgTwemojiAsset;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("from_emoji(BUCKET)", |b| b.iter(|| SvgTwemojiAsset::from_emoji(black_box("🪣"))));
    c.bench_function("from_emoji(DUCK)", |b| b.iter(|| SvgTwemojiAsset::from_emoji(black_box("🦆"))));
    c.bench_function("from_emoji(WARNING)", |b| b.iter(|| SvgTwemojiAsset::from_emoji(black_box("⚠️"))));
    c.bench_function("from_emoji(HEART ON FIRE)", |b| b.iter(|| SvgTwemojiAsset::from_emoji(black_box("❤️‍🔥"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
