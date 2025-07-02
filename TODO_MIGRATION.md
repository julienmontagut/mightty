# Mightty → egui/wgpu Migration TODO

## Overview
This document tracks the step-by-step migration of Mightty from OpenGL/glutin to egui/wgpu architecture while achieving platform uniformity between Linux and macOS.

## Migration Progress

### Phase 1: Preparation & Analysis (1-2 weeks)

#### Step 1.1: Dependency Analysis & Cleanup
- [ ] **Analyze current dependencies**
  - [ ] Document all platform-specific dependencies in `Cargo.toml`
  - [ ] Identify OpenGL/glutin usage patterns
  - [ ] Map winit usage throughout codebase

- [ ] **Update Cargo.toml**
  - [ ] Remove `glutin` dependency
  - [ ] Remove `gl_generator` dependency
  - [ ] Remove platform-specific `objc2` dependencies
  - [ ] Add `egui = "0.24"`
  - [ ] Add `eframe = { version = "0.24", features = ["default_fonts", "glow", "persistence"] }`
  - [ ] Update `winit` to compatible version with egui

- [ ] **Update build.rs**
  - [ ] Remove OpenGL binding generation code
  - [ ] Keep version/git hash generation
  - [ ] Test build system still works

#### Step 1.2: Create Abstraction Layer
- [ ] **Create platform module structure**
  - [ ] Create `src/platform/mod.rs`
  - [ ] Create `src/platform/macos.rs`
  - [ ] Create `src/platform/linux.rs`

- [ ] **Define PlatformServices trait**
  - [ ] `get_process_cwd(pid: i32) -> Result<PathBuf, std::io::Error>`
  - [ ] `set_locale_environment()`
  - [ ] `get_default_font() -> &'static str`
  - [ ] `get_default_shell() -> Option<&'static str>`
  - [ ] `get_url_opener() -> &'static str`

- [ ] **Initial trait implementations**
  - [ ] Stub implementations for both platforms
  - [ ] Compile-time platform selection logic
  - [ ] Basic integration tests

### Phase 2: Core Infrastructure Migration (2-3 weeks)

#### Step 2.1: Replace Event Loop
- [ ] **Create eframe application structure**
  - [ ] Create `MighttyApp` struct implementing `eframe::App`
  - [ ] Move terminal state into app struct
  - [ ] Implement basic `update()` method

- [ ] **Update main.rs**
  - [ ] Replace winit EventLoop with `eframe::run_native()`
  - [ ] Update command-line argument handling
  - [ ] Preserve existing CLI interface
  - [ ] Test basic app startup

- [ ] **Update event.rs**
  - [ ] Replace `Processor` event handling with egui patterns
  - [ ] Map winit events to egui events where needed
  - [ ] Preserve terminal event processing logic

#### Step 2.2: Replace Renderer Architecture
- [ ] **Create egui renderer module**
  - [ ] Create `src/renderer/egui_renderer.rs`
  - [ ] Design `TerminalWidget` struct
  - [ ] Implement `egui::Widget` trait for terminal

- [ ] **Implement terminal rendering**
  - [ ] Text rendering using egui's text layout
  - [ ] Cursor rendering as egui shapes
  - [ ] Selection highlighting
  - [ ] Color and styling support

- [ ] **Update renderer/mod.rs**
  - [ ] Switch from OpenGL renderer to egui renderer
  - [ ] Preserve existing renderer interface where possible
  - [ ] Update texture handling for egui

### Phase 3: Input System Migration (1-2 weeks)

#### Step 3.1: Replace Input Handling
- [ ] **Update input/mod.rs**
  - [ ] Replace winit input event handling with egui input
  - [ ] Implement `handle_input(&mut self, ctx: &egui::Context)`
  - [ ] Map egui events to terminal actions

- [ ] **Update keyboard.rs**
  - [ ] Replace `KeyEvent` handling with egui key events
  - [ ] Preserve terminal keyboard sequences
  - [ ] Handle special key combinations
  - [ ] Test all keyboard shortcuts

- [ ] **Update mouse handling**
  - [ ] Replace winit mouse events with egui mouse events
  - [ ] Implement mouse selection
  - [ ] Handle mouse wheel scrolling
  - [ ] Preserve click-to-focus behavior

#### Step 3.2: Platform-Specific Input Unification
- [ ] **Remove macOS-specific input code**
  - [ ] Remove Option-as-Alt handling or implement in egui layer
  - [ ] Use egui's cross-platform input handling
  - [ ] Test input behavior on both platforms

- [ ] **Unify clipboard handling**
  - [ ] Replace platform-specific clipboard with egui clipboard
  - [ ] Update `src/clipboard.rs`
  - [ ] Test copy/paste functionality
  - [ ] Preserve selection clipboard on Linux

### Phase 4: Window Management Migration (1-2 weeks)

#### Step 4.1: Replace Window Management
- [ ] **Update display/window.rs**
  - [ ] Simplify window management using eframe
  - [ ] Remove direct winit window manipulation
  - [ ] Preserve essential window operations

- [ ] **Update display/mod.rs**
  - [ ] Integrate with eframe window management
  - [ ] Update display initialization
  - [ ] Test window creation and management

#### Step 4.2: Feature Parity Assessment
- [ ] **Handle macOS-specific features**
  - [ ] Simple fullscreen → eframe fullscreen API
  - [ ] Window tabs → Remove or implement as egui tabs
  - [ ] Window shadows → Use eframe defaults
  - [ ] Focus behavior → Use eframe defaults

- [ ] **Test window features**
  - [ ] Fullscreen toggle
  - [ ] Window resizing
  - [ ] Title updates
  - [ ] Transparency support (if needed)

### Phase 5: Configuration System Migration (1 week)

#### Step 5.1: Unify Configuration
- [ ] **Update config/defaults.rs**
  - [ ] Remove `#[cfg(target_os)]` conditionals
  - [ ] Use platform abstraction layer
  - [ ] Create `PlatformDefaults` struct

- [ ] **Update config/window.rs**
  - [ ] Simplify window configuration for egui
  - [ ] Remove macOS-specific window options
  - [ ] Test configuration loading

- [ ] **Update config/font.rs**
  - [ ] Adapt font configuration for egui font system
  - [ ] Test font loading and rendering
  - [ ] Preserve custom font support

### Phase 6: Terminal Core Integration (2-3 weeks)

#### Step 6.1: Integrate Terminal with egui
- [ ] **Update terminal/mod.rs**
  - [ ] Integrate terminal rendering with egui
  - [ ] Preserve terminal state management
  - [ ] Update terminal event handling

- [ ] **Create terminal widgets**
  - [ ] Implement scrollable terminal view
  - [ ] Handle terminal resizing
  - [ ] Implement search overlay as egui window

#### Step 6.2: Text Rendering Migration
- [ ] **Replace glyph caching**
  - [ ] Use egui's text layout system
  - [ ] Implement efficient text rendering
  - [ ] Handle different font styles and sizes

- [ ] **Implement cursor rendering**
  - [ ] Draw cursor using egui shapes
  - [ ] Handle cursor blinking
  - [ ] Support different cursor styles

- [ ] **Test terminal rendering**
  - [ ] Verify text output correctness
  - [ ] Test with various terminal programs
  - [ ] Performance testing

### Phase 7: Platform Services Implementation (1-2 weeks)

#### Step 7.1: Implement Linux Platform Services
- [ ] **Complete LinuxPlatform implementation**
  - [ ] Implement `get_process_cwd()` using `/proc`
  - [ ] Implement `set_locale_environment()`
  - [ ] Set appropriate defaults for Linux

- [ ] **Test Linux implementation**
  - [ ] Test process working directory detection
  - [ ] Test locale settings
  - [ ] Test default configurations

#### Step 7.2: Implement macOS Platform Services
- [ ] **Complete MacOSPlatform implementation**
  - [ ] Integrate existing `macos::proc::cwd()` logic
  - [ ] Integrate existing `macos::locale` logic
  - [ ] Set appropriate defaults for macOS

- [ ] **Test macOS implementation**
  - [ ] Test process working directory detection
  - [ ] Test locale settings
  - [ ] Test default configurations

### Phase 8: Cleanup & Testing (1-2 weeks)

#### Step 8.1: Remove Old Code
- [ ] **Delete obsolete files**
  - [ ] Remove `src/renderer/gl/` directory
  - [ ] Remove OpenGL-specific code
  - [ ] Clean up unused display logic

- [ ] **Clean up conditionals**
  - [ ] Remove all `#[cfg(target_os = "...")]` conditionals
  - [ ] Update imports throughout codebase
  - [ ] Remove unused dependencies

#### Step 8.2: Update Build System
- [ ] **Update Cargo.toml**
  - [ ] Final dependency cleanup
  - [ ] Remove unused features
  - [ ] Optimize dependency versions

- [ ] **Update CI/CD**
  - [ ] Update `.github/workflows/ci.yml`
  - [ ] Test builds on both platforms
  - [ ] Update release workflow if needed

- [ ] **Update documentation**
  - [ ] Update `README.md` if needed
  - [ ] Update `INSTALL.md` for new dependencies
  - [ ] Update configuration documentation

### Phase 9: Feature Restoration & Polish (1-2 weeks)

#### Step 9.1: Restore Missing Features
- [ ] **Clipboard integration**
  - [ ] Implement using egui's clipboard API
  - [ ] Test copy/paste functionality
  - [ ] Test selection clipboard behavior

- [ ] **Configuration hot-reload**
  - [ ] Integrate with egui's frame callback system
  - [ ] Test configuration file watching
  - [ ] Preserve existing config monitor functionality

- [ ] **Theme support**
  - [ ] Implement using egui's style system
  - [ ] Test light/dark theme switching
  - [ ] Preserve custom color schemes

#### Step 9.2: Performance Optimization
- [ ] **Profile performance**
  - [ ] Benchmark egui rendering vs old OpenGL
  - [ ] Identify performance bottlenecks
  - [ ] Optimize text rendering if needed

- [ ] **Implement damage tracking**
  - [ ] Efficient terminal updates
  - [ ] Minimize unnecessary redraws
  - [ ] Test with high-throughput terminal output

- [ ] **Memory optimization**
  - [ ] Profile memory usage
  - [ ] Optimize texture usage
  - [ ] Test long-running stability

## Testing Checklist

### Functional Testing
- [ ] **Basic terminal functionality**
  - [ ] Text input and output
  - [ ] Cursor movement and positioning
  - [ ] Terminal scrolling
  - [ ] Color and formatting support

- [ ] **Advanced terminal features**
  - [ ] Search functionality
  - [ ] Selection and copy/paste
  - [ ] Multiple windows/tabs
  - [ ] Configuration reloading

- [ ] **Platform-specific testing**
  - [ ] Test on macOS
  - [ ] Test on Linux (multiple distros if possible)
  - [ ] Test with different window managers
  - [ ] Verify keyboard shortcuts work on both platforms

### Performance Testing
- [ ] **Rendering performance**
  - [ ] Compare FPS with old implementation
  - [ ] Test with large terminal buffers
  - [ ] Test with high-frequency updates

- [ ] **Memory usage**
  - [ ] Profile memory consumption
  - [ ] Test for memory leaks
  - [ ] Compare with old implementation

### Regression Testing
- [ ] **Existing functionality preserved**
  - [ ] All command-line arguments work
  - [ ] Configuration files load correctly
  - [ ] Terminal programs work correctly
  - [ ] IPC functionality preserved

## Migration Notes

### Known Issues to Address
- [ ] egui text rendering differences from OpenGL
- [ ] Input timing differences
- [ ] Window management behavior changes
- [ ] Performance characteristics

### Platform Differences to Resolve
- [ ] Font rendering consistency
- [ ] Input handling edge cases
- [ ] Window focus behavior
- [ ] Clipboard behavior differences

## Completion Criteria

### Phase Completion
- [ ] All phase tasks completed
- [ ] Tests passing on both platforms
- [ ] No regressions in core functionality
- [ ] Performance acceptable
- [ ] Code review completed

### Migration Success
- [ ] Single codebase for both platforms
- [ ] No platform-specific conditionals in core logic
- [ ] Feature parity maintained
- [ ] Performance equal or better than original
- [ ] All tests passing
- [ ] Documentation updated

---

**Migration Start Date:** _[To be filled]_
**Target Completion Date:** _[To be filled]_
**Current Phase:** _[To be updated]_