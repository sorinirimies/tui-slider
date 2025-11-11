# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of tui-slider
- Horizontal and vertical slider orientations
- Multiple built-in visual styles (default, volume, audio_level, vu_meter, equalizer, minimal, modern, fire, ocean, rainbow)
- Smooth animations with configurable easing functions
- Support for 15 different easing functions including linear, cubic, quadratic, exponential, bounce, and elastic
- Color gradient system for sliders with multiple preset gradients
- Customizable symbols for handles and bars
- Interactive and display-only modes
- State management with automatic value clamping
- Animation system with duration and easing control
- Comprehensive example applications:
  - Horizontal slider showcase
  - Vertical equalizer
  - Audio level meters with VU meters
  - Complete music player interface
  - 10-band graphic equalizer
  - Custom styles showcase

### Features
- SliderState for managing value, bounds, and animations
- SliderOrientation (Horizontal/Vertical)
- SliderStyle with multiple preset themes
- SliderSymbols with Unicode and ASCII symbol sets
- BarStyle with gradient support
- HandleStyle with highlight states
- ColorGradient with smooth color interpolation
- AnimationConfig with multiple easing options
- Full ratatui integration as a Widget

### Documentation
- Comprehensive README with examples
- API documentation with examples
- Multiple working example applications
- Quick start guide

## [0.1.0] - 2024-01-XX

Initial release.