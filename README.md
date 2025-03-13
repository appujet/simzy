# string-proximity. - Fast String Similarity Library for Node.js

[![CI](-https://github.com/Mintone-creators/string-proximity/actions/workflows/CI.yml/badge.svg)](-https://github.com/Mintone-creators/string-proximity/actions/workflows/CI.yml)  
[![npm version](https://badge.fury.io/js/string-proximity.svg)](https://badge.fury.io/js/string-proximity)  
[![GitHub Repo stars](https://img.shields.io/github/stars/appujet/string-proximity?style=social)](-https://github.com/Mintone-creators/string-proximity)

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Testing](#testing)
- [Performance](#performance)
- [License](#license)
- [Contributing](#contributing)

## Overview

**string-proximity.** is a high-performance string similarity library for Node.js, powered by Rust through NAPI bindings. Designed for speed and accuracy, string-proximity. supports both Unicode and ASCII characters. Whether you're building search engines, fuzzy matchers, or duplicate detectors, string-proximity. delivers precise similarity calculations without sacrificing ease of use.

## Features

- **Levenshtein Distance**  
  Calculate the minimum number of edits (insertions, deletions, or substitutions) required to transform one string into another, with full Unicode support.

- **Jaro-Winkler Similarity**  
  Detect typos and perform fuzzy matching by measuring similarity based on common prefixes and transpositions.

- **Best Match Finder**  
  Quickly identify the closest match from a list of candidates, complete with confidence filtering to dismiss weak matches.

## Installation

Install string-proximity. using your preferred package manager:

```bash
npm install string-proximity
# or
yarn add string-proximity
# or
pnpm add string-proximity
```

## Usage

string-proximity. is built to be intuitive and simple to integrate into your projects.

### Calculating String Similarity

```javascript
import { stringSimilarity, SimilarityAlgorithm } from 'string-proximity';

const score = stringSimilarity('hello', 'helo', SimilarityAlgorithm.JaroWinkler);
console.log(`Similarity Score: ${score}`); // Example output: 0.96
```

### Computing Levenshtein Distance

```javascript
import { levenshteinDistance } from 'string-proximity';

const distance = levenshteinDistance('kitten', 'sitting');
console.log(`Levenshtein Distance: ${distance}`); // Example output: 3
```

### Finding the Best Match

```javascript
import { findBestMatch } from 'string-proximity';

const result = findBestMatch('apple', ['apples', 'aple', 'apple']);
console.log(`Best match: ${result.best_match.string}`);
console.log(`Score: ${result.best_match.score}`);
```

## Testing

string-proximity. comes with a comprehensive suite of tests using AVA to ensure reliability and performance. Run the tests with:

```bash
npm test
```

## Performance

Powered by Rust, string-proximity. outperforms traditional JavaScript implementations for string similarity tasks. Whether you're comparing short strings or processing large volumes of text, string-proximity. delivers the speed and accuracy your application needs.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! If you have ideas for improvements, encounter bugs, or want to help optimize performance, please open an issue or submit a pull request. Together, we can make string-proximity. even better.
