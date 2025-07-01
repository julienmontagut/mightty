# Mightty Simplification & Modernization Plan

## Overview
Migrate Mightty from custom OpenGL/glutin architecture to modern egui/eframe + wgpu while reducing codebase by 50-70% and supporting only macOS/Linux with vim mode preserved.

**Target**: 14,700 LOC → ~5,450 LOC (63% reduction)
**Timeline**: 12 weeks
**Scope**: Terminal emulator core + vim mode + basic configuration

## Dependency Graph & Critical Path

```
Phase 1 (Platform) → Phase 2A (Dependencies) → Phase 2B (Architecture)
                                             ↓
Phase 3 (Config) ← Phase 2C (Rendering) ← Phase 2B
                                             ↓
                  Phase 4 (Features) ← Phase 2C
                                             ↓
                           Phase 5 (Testing) ← Phase 4
```

## Phase 1: Platform Cleanup (Week 1)
**Dependencies**: None - Safe to start immediately
**Risk**: Low - No functional changes

### 1.1 Remove Unused Platforms
- [x] **P1.1.1** Remove Windows references from CI/docs (30 min)
- [x] **P1.1.2** Remove BSD support from `.builds/freebsd.yml` (15 min)
- [x] **P1.1.3** Audit and remove `#[cfg(target_os = "freebsd")]` blocks (2 hours)
  ```bash
  grep -r "freebsd" src/ --include="*.rs"
  grep -r "windows" src/ --include="*.rs" | grep cfg
  ```

### 1.2 Simplify Platform Differences
- [x] **P1.2.1** Create `src/config/defaults.rs` (4 hours)
- [x] **P1.2.2** Replace runtime platform detection in:
  - `src/clipboard.rs` (1 hour)
  - `src/config/ui_config.rs` (2 hours)
  - `src/daemon.rs` (1 hour)

**Deliverable**: Codebase works identically but with simplified platform handling
**LOC Reduction**: ~200 LOC

---

## Phase 2A: New Dependencies Setup (Week 2)
**Dependencies**: Phase 1 complete
**Risk**: Low - Additive changes only

### 2A.1 Add New Dependencies
- [ ] **P2A.1.1** Update `Cargo.toml` - Add egui ecosystem (30 min)
  ```toml
  [dependencies]
  eframe = "0.24"
  egui = "0.24"
  egui_wgpu = "0.24"
  wgpu = "0.18"
  # Keep existing deps for now - will remove in Phase 2C
  ```
- [ ] **P2A.1.2** Verify compilation with new deps (1 hour)
- [ ] **P2A.1.3** Create feature flags for migration (2 hours)
  ```toml
  [features]
  default = ["old-renderer"]
  old-renderer = ["glutin", "crossfont"]
  new-renderer = ["eframe", "egui_wgpu"]
  ```

**Deliverable**: Project compiles with both old and new dependencies
**LOC Change**: +50 LOC (Cargo.toml + feature flags)

---

## Phase 2B: New Architecture Foundation (Week 3)
**Dependencies**: Phase 2A complete
**Risk**: Medium - Core architecture changes

### 2B.1 Create New App Structure
- [ ] **P2B.1.1** Create `src/app.rs` - Main egui application (6 hours)
  ```rust
  pub struct MighttyApp {
      terminal: Arc<Mutex<Term>>,
      vim_mode: VimMode,
      config: SimpleConfig,
      font_id: egui::FontId,
  }

  impl eframe::App for MighttyApp {
      fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
          // Terminal rendering will be implemented in Phase 2C
      }
  }
  ```

- [ ] **P2B.1.2** Create `src/terminal_widget.rs` - egui terminal widget (8 hours)
  ```rust
  pub struct TerminalWidget<'a> {
      terminal: &'a mut Term,
      vim_mode: &'a VimMode,
      config: &'a SimpleConfig,
  }

  impl egui::Widget for TerminalWidget<'_> {
      fn ui(self, ui: &mut egui::Ui) -> egui::Response {
          // Implementation in Phase 2C
          ui.label("Terminal placeholder")
      }
  }
  ```

- [ ] **P2B.1.3** Create parallel main entry point (4 hours)
  ```rust
  // src/main.rs - Add feature-gated entry points
  #[cfg(feature = "new-renderer")]
  fn main_egui() -> Result<(), Box<dyn Error>> {
      let options = eframe::NativeOptions::default();
      eframe::run_native("Mightty", options, Box::new(|_cc| Box::new(MighttyApp::new())))
  }

  #[cfg(feature = "old-renderer")]
  fn main_glutin() -> Result<(), Box<dyn Error>> {
      // Existing main function
  }
  ```

### 2B.2 Terminal Core Interface
- [ ] **P2B.2.1** Create clean terminal interface (4 hours)
  ```rust
  // src/terminal/interface.rs
  pub trait TerminalCore {
      fn render_cells(&self) -> Vec<RenderableCell>;
      fn handle_input(&mut self, input: &str);
      fn resize(&mut self, cols: u16, rows: u16);
      fn scroll(&mut self, delta: i32);
  }
  ```

**Deliverable**: Parallel architecture exists, can switch between old/new with features
**LOC Addition**: +800 LOC (new architecture)

---

## Phase 2C: Implement egui Rendering (Week 4-5)
**Dependencies**: Phase 2B complete
**Risk**: High - Core rendering functionality

### 2C.1 Terminal Text Rendering
- [ ] **P2C.1.1** Implement basic character rendering in egui (12 hours)
  ```rust
  // In terminal_widget.rs
  impl egui::Widget for TerminalWidget<'_> {
      fn ui(self, ui: &mut egui::Ui) -> egui::Response {
          let cells = self.terminal.render_cells();
          for cell in cells {
              // Render each character with egui::TextEdit or custom painter
          }
      }
  }
  ```

- [ ] **P2C.1.2** Handle terminal colors and styling (8 hours)
  - ANSI color support
  - Bold/italic/underline rendering
  - Background colors

- [ ] **P2C.1.3** Implement cursor rendering (4 hours)
  - Different cursor shapes (block, beam, underline)
  - Cursor blinking

### 2C.2 Input Handling
- [ ] **P2C.2.1** Basic keyboard input (6 hours)
  ```rust
  // Capture egui input events and convert to terminal input
  if ui.input().key_pressed(egui::Key::Enter) {
      terminal.handle_input("\r\n");
  }
  ```

- [ ] **P2C.2.2** Special key handling (4 hours)
  - Arrow keys, function keys
  - Ctrl/Cmd combinations
  - Alt key sequences

### 2C.3 Vim Mode Integration
- [ ] **P2C.3.1** Integrate existing vim mode with egui input (8 hours)
- [ ] **P2C.3.2** Vim visual feedback in egui (4 hours)
  - Mode indicator
  - Selection highlighting
  - Command line display

**Deliverable**: Basic functional terminal with egui rendering
**LOC Addition**: +1200 LOC
**LOC Reduction**: Can now remove old renderer in next phase

---

## Phase 3: Configuration Simplification (Week 6)
**Dependencies**: Phase 2C working (need new app structure)
**Risk**: Medium - May break existing configs

### 3.1 Create Simple Config System
- [ ] **P3.1.1** Design minimal config structure (2 hours)
  ```rust
  // src/config/simple.rs
  #[derive(Deserialize, Default)]
  pub struct SimpleConfig {
      pub font_family: String,
      pub font_size: f32,
      pub theme: ColorTheme,
      pub shell: Option<String>,
      pub opacity: f32,
  }
  ```

- [ ] **P3.1.2** Update config loading (4 hours)

### 3.2 Remove Complex Config System
- [ ] **P3.2.1** Remove unused config files (2 hours)
  - `src/config/bell.rs`
  - `src/config/cursor.rs`
  - `src/config/debug.rs`
  - `src/config/monitor.rs`
  - `src/config/mouse.rs`
  - `src/config/scrolling.rs`
  - `src/config/selection.rs`
  - `src/config/serde_utils.rs`

- [ ] **P3.2.2** Simplify remaining config files (6 hours)
  - `src/config/mod.rs` - Remove figment, IPC config
  - `src/config/font.rs` - Basic font family/size only
  - `src/config/color.rs` - Essential colors only
  - `src/config/window.rs` - Basic window options

- [ ] **P3.2.3** Update new app to use simple config (4 hours)

**Deliverable**: Simple, maintainable configuration system
**LOC Reduction**: ~1700 LOC

---

## Phase 4A: Remove Old Rendering System (Week 7)
**Dependencies**: Phase 2C complete (new rendering working)
**Risk**: High - Point of no return

### 4A.1 Remove OpenGL/glutin Dependencies
- [ ] **P4A.1.1** Remove old renderer code (2 hours)
  - Delete `src/renderer/` directory (~2500 LOC)
  - Delete `src/display/` directory (~3000 LOC)

- [ ] **P4A.1.2** Remove graphics dependencies (1 hour)
  ```toml
  # Remove from Cargo.toml:
  # glutin = "0.32.2"
  # crossfont = "0.8.1"
  # gl_generator = "0.14.0"
  ```

- [ ] **P4A.1.3** Remove build.rs (30 min)
  - No more OpenGL code generation needed

- [ ] **P4A.1.4** Update main.rs (2 hours)
  - Remove old entry point
  - Remove glutin event loop
  - Remove gl module imports

### 4A.2 Update Window Management
- [ ] **P4A.2.1** Remove custom window handling (4 hours)
  - Delete `src/window_context.rs`
  - Delete `src/display/window.rs`
  - Use eframe's window management exclusively

**Deliverable**: Single rendering path using egui/wgpu only
**LOC Reduction**: ~5500 LOC

---

## Phase 4B: Feature Simplification (Week 8-9)
**Dependencies**: Phase 4A complete (old code removed)
**Risk**: Medium - Feature removals

### 4B.1 Remove Non-Essential Features
- [ ] **P4B.1.1** Remove complex systems (6 hours)
  - Delete `src/message_bar.rs`
  - Delete `src/ipc.rs`
  - Delete `src/daemon.rs`
  - Remove hints system from config
  - Remove tab system

- [ ] **P4B.1.2** Simplify input system (8 hours)
  - `src/input/` - Remove complex keybinding system
  - Hardcode essential vim motions
  - Remove mouse hint interactions
  - Keep basic mouse selection

- [ ] **P4B.1.3** Simplify event system (6 hours)
  - `src/event.rs` - Remove IPC event handling
  - Remove complex window management
  - Remove multi-window support

### 4B.2 Preserve Essential Features
- [ ] **P4B.2.1** Ensure vim mode works completely (8 hours)
  - Test all vim motions
  - Test vim selection
  - Test vim search (basic)
  - Test mode transitions

- [ ] **P4B.2.2** Ensure basic terminal functions (4 hours)
  - Copy/paste works
  - Font size adjustment
  - Scrolling
  - Color support

**Deliverable**: Simplified but fully functional terminal
**LOC Reduction**: ~2000 LOC

---

## Phase 5: Testing & Polish (Week 10-12)
**Dependencies**: Phase 4B complete (all major changes done)
**Risk**: Low - Quality assurance

### 5.1 Comprehensive Testing
- [ ] **P5.1.1** Terminal compatibility testing (16 hours)
  - Test common CLI tools (vim, htop, etc.)
  - Test color support (256 colors, true color)
  - Test terminal sequences (escape codes)
  - Test different shells (bash, zsh, fish)

- [ ] **P5.1.2** Vim mode testing (12 hours)
  - Test all motion commands
  - Test selection modes
  - Test copy/paste in vim mode
  - Test search functionality

- [ ] **P5.1.3** Platform testing (8 hours)
  - Test on macOS (Intel + Apple Silicon)
  - Test on Linux (Ubuntu, Arch)
  - Test font rendering on both platforms

### 5.2 Performance Optimization
- [ ] **P5.2.1** Profile egui rendering performance (8 hours)
- [ ] **P5.2.2** Optimize terminal widget rendering (8 hours)
- [ ] **P5.2.3** Memory usage optimization (4 hours)

### 5.3 Documentation & Polish
- [ ] **P5.3.1** Update documentation (8 hours)
  - README.md with new feature set
  - Configuration guide
  - Vim mode keybindings
  - Installation instructions

- [ ] **P5.3.2** Code cleanup (8 hours)
  - Remove dead code
  - Update comments
  - Fix clippy warnings
  - Format code consistently

**Deliverable**: Production-ready simplified terminal emulator

---

## Risk Mitigation & Rollback Plans

### Critical Checkpoints
1. **After Phase 2B**: Must have working parallel architecture
2. **After Phase 2C**: Must have basic functional egui terminal
3. **After Phase 4A**: Point of no return - old renderer removed

### Rollback Procedures
- **Before Phase 4A**: Can revert to old renderer using feature flags
- **Phase 2-3**: Maintain git tags for major milestones
- **Testing**: Each phase must pass basic smoke tests before proceeding

### Risk Assessment
- **Highest Risk**: Phase 2C (egui rendering) and Phase 4A (removing old code)
- **Medium Risk**: Phase 3 (config changes), Phase 4B (feature removal)
- **Lowest Risk**: Phase 1 (platform cleanup), Phase 5 (testing)

---

## Success Metrics

### Code Reduction Targets
| Component | Before | After | Reduction |
|-----------|--------|-------|-----------|
| Rendering | 5500 LOC | 500 LOC | 91% |
| Config | 2000 LOC | 300 LOC | 85% |
| Platform | 400 LOC | 100 LOC | 75% |
| Features | 2000 LOC | 600 LOC | 70% |
| Input/Events | 1000 LOC | 400 LOC | 60% |
| Terminal Core | 4000 LOC | 3200 LOC | 20% |
| **Total** | **14700 LOC** | **5450 LOC** | **63%** |

### Functional Requirements
- ✅ All basic terminal emulation works
- ✅ Vim mode fully functional
- ✅ Copy/paste works on both platforms
- ✅ Font rendering quality maintained
- ✅ Performance comparable to original
- ✅ Simple configuration system works

### Performance Targets
- Startup time: <500ms (vs current ~300ms)
- Memory usage: <50MB for basic usage
- Rendering: 60fps for normal terminal usage
- Input latency: <16ms

---

## Dependencies Summary

```
Phase 1 → Phase 2A → Phase 2B → Phase 2C → Phase 4A → Phase 4B → Phase 5
    ↑                    ↓                      ↑
    └─ Can work on ──────┴─ Phase 3 ──────────┘
```

**Critical Path**: 1 → 2A → 2B → 2C → 4A (10 weeks minimum)
**Parallel Work**: Phase 3 can start after 2B completes
**Buffer Time**: 2 weeks built in for unexpected issues

This plan ensures a systematic migration with clear checkpoints and rollback options while achieving the target code reduction and modernization goals.
