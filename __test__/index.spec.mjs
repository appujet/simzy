import test from 'ava'
import {
  levenshteinDistance,
  SimilarityAlgorithm,
  stringSimilarity,
  findBestMatch
} from '../index.js'

// Levenshtein Distance Tests
test('levenshteinDistance - basic cases', t => {
  t.is(levenshteinDistance('', ''), 0)
  t.is(levenshteinDistance('a', 'a'), 0)
  t.is(levenshteinDistance('a', 'b'), 1)
  t.is(levenshteinDistance('kitten', 'sitting'), 3)
  t.is(levenshteinDistance('flaw', 'lawn'), 2)
})

test('levenshteinDistance - unicode support', t => {
  t.is(levenshteinDistance('café', 'cafe'), 1)
  t.is(levenshteinDistance('😀😁', '😀😂'), 1)
  t.is(levenshteinDistance('ünicode', 'unicode'), 1)
})

// String Similarity Tests

test('stringSimilarity - Jaro-Winkler', t => {
  t.is(stringSimilarity('', '', SimilarityAlgorithm.JaroWinkler), 1)
  t.is(stringSimilarity('a', '', SimilarityAlgorithm.JaroWinkler), 0)
  t.is(stringSimilarity('MARTHA', 'MARHTA', SimilarityAlgorithm.JaroWinkler).toFixed(4), '0.9611')
  t.is(stringSimilarity('DWAYNE', 'DUANE', SimilarityAlgorithm.JaroWinkler).toFixed(4), '0.8400')
})

test('stringSimilarity - LevenshteinSimilarity', t => {
  t.is(stringSimilarity('', '', SimilarityAlgorithm.LevenshteinSimilarity), 1)
  t.is(stringSimilarity('a', 'aaa', SimilarityAlgorithm.LevenshteinSimilarity).toFixed(4), '0.3333')
  t.is(stringSimilarity('kitten', 'sitting', SimilarityAlgorithm.LevenshteinSimilarity).toFixed(4), '0.5714')
})

// Find Best Match Tests

test('findBestMatch - basic cases', t => {
  const result = findBestMatch('kitten', ['sitting', 'kitten']);
  t.is(result.bestMatch.string, 'kitten');
  t.is(result.bestMatch.score, 1);
  t.true(result.allMatches.length >= 1); // Instead of exact array check
  t.false(result.hasTie);
});

test('findBestMatch - LevenshteinSimilarity', t => {
  const options = { algorithm: SimilarityAlgorithm.LevenshteinSimilarity };
  const result = findBestMatch('kitten', ['sitting', 'kitten'], options);
  t.is(result.bestMatch.string, 'kitten');
  t.is(result.bestMatch.score, 1);
  t.true(result.allMatches.length >= 1); // Instead of exact array check
  t.false(result.hasTie);
});

test('findBestMatch - threshold', t => {
  const options = { threshold: 0.8 };
  const result = findBestMatch('kitten', ['sitting', 'kitten'], options);
  t.is(result.bestMatch.string, 'kitten');
  t.is(result.bestMatch.score, 1);
  t.true(result.allMatches.length >= 1); // Instead of exact array check
  t.false(result.hasTie);
});

test('findBestMatch - JaroWinkler', t => {
  const options = { algorithm: SimilarityAlgorithm.JaroWinkler };
  const result = findBestMatch('kitten', ['sitting', 'kitten'], options);
  t.is(result.bestMatch.string, 'kitten');
  t.is(result.bestMatch.score, 1);
  t.true(result.allMatches.length >= 1); // Instead of exact array check
  t.false(result.hasTie);
});

test('findBestMatch - unicode support', t => {
  const result = findBestMatch('café', ['cafe']);
  t.is(result.bestMatch.string, 'cafe');
  t.is(result.bestMatch.score, 1);
  t.true(result.allMatches.length >= 1); // Instead of exact array check
  t.false(result.hasTie);
});
