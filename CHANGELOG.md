# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
...

## [0.0.1] - 2021-10-31

### Added
- Clearing display
- Showing the current I2C address
- Setting the I2C address (requires more testing)
- Setting brightness level
- Setting display mode (rotate/scroll)
- Displaying a single digit at a given position
- Sending a single digit to the display
- Displaying a single character at a given position
- Sending a single character to the display
- Displaying a number using all the digits (with leading zeros)
- Displaying temperature with a chosen unit, no leading zeros, minus sign, lower and upper threshold, LL/HH if below/over threshold, ---- if exceeding -99/999
- Displaying humidity between 0 and 100 with settable lower/upper threshold
- Displaying time in HH.MM format with an optional dot

[0.0.1]: https://github.com/nebelgrau77/akafugu_twidisplay-rs/releases/tag/v0.0.1