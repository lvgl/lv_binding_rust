<h1 align="center"> LittlevGL - Open-source Embedded GUI Library in Rust</h1>

![Original LittlevGL demo image](lv_demo.png)

<p align="center">
LittlevGL provides everything you need to create a Graphical User Interface (GUI) on embedded systems with easy-to-use graphical elements, beautiful visual effects and low memory footprint. 
</p>
<p align="center">
LittlevGL is compatible with <samp>#![no_std]</samp> environments by default.
</p>

<h4 align="center">
<a href="https://lvgl.io/">Official LittlevGL Website </a> &middot; 
<a href="https://github.com/littlevgl/lvgl">C library repository</a> &middot;
<a href="https://lvgl.io/demos">Live demo</a>
</h4>

---

![Rust bindings usage demo code.](demo.png)

## Usage

Edit your `Cargo.toml` file dependencies with:
```
$ cargo add lvgl
```

The build requires the environment variable bellow to be set:

- `DEP_LV_CONFIG_PATH`: Path to the directory containing the `lv_conf.h` header file used for configuration of LittlevGL library.

We recommend the `lv_conf.h` file to be in your project's root directory. If so, the command to build your project would be:
```shell script
$ DEP_LV_CONFIG_PATH=`pwd` cargo build
```

### Building for embedded environments

We make use of `bindgen` for generating the bindings to LittlevGL at build time. There is a problem in cargo when building
for `no_std`, so we need to use a workaround to build "lvgl-rs". The mainstrem issue in cargo is being tracked at
[rust-lang/cargo#7915](https://github.com/rust-lang/cargo/issues/7915).

```shell
$ DEP_LV_CONFIG_PATH=`pwd` cargo build -Zfeatures=build_dep
```

#### Requirements / Limitations

LittlevGL C libary do allocate memory dynamically and we need to allocate memory on the heap in the Rust side as well
([`Box`](https://doc.rust-lang.org/beta/alloc/boxed/struct.Box.html)).
That is required, so we can safely provide Rust pointers through FFI. For that reason, we do require
[`alloc`](https://doc.rust-lang.org/alloc/) module to be available.

## Running the demo

[This project contains examples that can run in a desktop simulator.](./examples)

First, make sure to pull `lvgl-rs` submodules:
```shell
$ git submodule init
$ git submodule update 
```

Then run the `demo` example:

```shell
$ DEP_LV_CONFIG_PATH=`pwd`/examples/include cargo run --example demo
```

## Feature Support

The bindings are still in development. There are many features of LVGL that needs to be exposed by `lvgl-rs`. In
this section you can check what is implemented at the moment.

### Features

List of LVGL features that impacts the library usage in general.
- [x] Displays: We use [`embedded_graphics`](https://docs.rs/embedded-graphics/0.6.2/embedded_graphics/) library to
      draw to the display. You can use `lvgl-rs` with any of the
      [`embedded_graphics` supported displays](https://docs.rs/embedded-graphics/0.6.2/embedded_graphics/#supported-displays).
- [x] Events: You can listen and trigger events in widget objects.
- [x] Styles: You can set styles in any exposed object. We are still missing the possibility of defining base styles.
- [ ] Input Devices
- [ ] Fonts
- [ ] Images
- [ ] File system
- [ ] Animations
- [ ] Tasks

### Widgets

- [x] Base object (lv_obj)
- [ ] Arc (lv_arc)
- [x] Bar (lv_bar)
- [x] Button (lv_btn)
- [ ] Button matrix (lv_btnm)
- [ ] Calendar (lv_calendar)
- [ ] Canvas (lv_canvas)
- [ ] Checkbox (lv_cb)
- [ ] Chart (lv_chart)
- [ ] Container (lv_cont)
- [ ] Color picker (lv_cpicker)
- [ ] Drop-down list (lv_ddlist)
- [ ] Gauge (lv_gauge)
- [ ] Image (lv_img)
- [ ] Image button (lv_imgbtn)
- [ ] Keyboard (lv_kb)
- [x] Label (lv_label)
- [ ] LED (lv_led)
- [ ] Line (lv_line)
- [ ] List (lv_list)
- [ ] Line meter (lv_lmeter)
- [ ] Message box (lv_mbox)
- [ ] Page (lv_page)
- [ ] Preloader (lv_preload)
- [ ] Roller (lv_roller)
- [ ] Slider (lv_slider)
- [ ] Spinbox (lv_spinbox)
- [ ] Switch (lv_sw)
- [ ] Table (lv_table)
- [ ] Tabview (lv_tabview)
- [ ] Text area (lv_ta)
- [ ] Tile view (lv_tileview)
- [ ] Window (lv_win)

Widgets currently implemented might have some missing features. If the widget you want to use is not exposed or
is missing a feature you want to make use, please send a Pull Request or open an issue.
