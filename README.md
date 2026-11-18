# ansi2html

Yet another ANSI to HTML converter

`ansi2html` converts ANSI escape sequences in terminal output into HTML, preserving text formatting and colors.

It is designed primarily as a supplement for iTerm2, handling some of its idiosyncratic ANSI escape sequence behavior that may not be handled correctly by existing ANSI-to-HTML converters.


## Goals

Existing ANSI-to-HTML converters provide useful functionality, but terminal output can contain variations in escape sequences depending on the commands and the terminal emulator we use. For example, the following command outputs colors not found in the ANSI pallet:

![color.png](<./color.png>)

This project aims to:

- Preserve terminal styling accurately in HTML.
- Handle iTerm2-specific ANSI behavior.
- Support a broad range of ANSI SGR color and text attributes.
- Keep color definitions customizable through CSS variables.
- Provide a simple interface that integrates easily into other tools and workflows.

## Features

- **ANSI color support**
  - Standard 8-color foreground and background colors
  - Bright 16-color foreground and background colors
  - 256-color palette
  - 24-bit truecolor (RGB)
- **Text attributes**
  - Bold
  - Italic
  - Underline
  - Reverse video
- **iTerm2 compatibility**
  - Handles iTerm2-specific extended RGB color sequences
  - Supports optional color space identifiers in truecolor sequences
  - Handles both colon-separated and semicolon-separated SGR parameters
- **HTML output**
  - Converts terminal colors and attributes into inline CSS
  - Uses CSS variables for the standard 16 ANSI colors, allowing themes to be customized
  - Escapes HTML special characters

## Dependency

This tool uses [vte](https://github.com/alacritty/vte) parser for ANSI escape sequence processing.

## Installation

Clone the repository:

```shell
git clone https://github.com/takahirokosaka/ansi2html-260414-ayqcj.git
cd ansi2html
```

Build:

```shell
cargo build --release
```

The executable will be available at:

```text
target/release/ansi2html
```

## CSS Variables

The standard 16 ANSI colors are emitted as CSS variables rather than hard-coded colors.

This allows the appearance of converted output to follow the user’s preferred color scheme.

Define the variables in your HTML:

```css
:root {
   --ansi-black: #000000;
   --ansi-red: #cd3131;
   --ansi-green: #00bc00;
   --ansi-yellow: #949800;
   --ansi-blue: #0451a5;
   --ansi-magenta: #bc05bc;
   --ansi-cyan: #0598bc;
   --ansi-white: #e5e5e5;

   --ansi-bright-black: #666666;
   --ansi-bright-red: #cd3131;
   --ansi-bright-green: #00ff00;
   --ansi-bright-yellow: #ffff00;
   --ansi-bright-blue: #5c5cff;
   --ansi-bright-magenta: #ff00ff;
   --ansi-bright-cyan: #00ffff;
   --ansi-bright-white: #ffffff;
}
```

You can customize these values to match your iTerm2 color scheme.

## Command-Line Usage

`ansi2html` reads terminal output from standard input and writes HTML to standard output.

For example:

```shell
printf '\033[1;31mHello, world!\033[0m\n' | cargo run
```

Output:

```HTML
<span style="color:var(--ansi-red); font-weight:700;">Hello, world!</span>
```

**Truecolor example**

```shell
printf '\033[38:2:1:30:144:255mHello\033[0m\n' | cargo run
```

The optional `1` in `38:2:1:30:144:255` is an iTerm2 color space identifier.

Output:

```HTML
<span style="color:#1e90ff;">Hello</span>
```

**Background color and text attributes**

```shell
printf '\033[1;3;4;48:2:127:0:127mStyled text\033[0m\n' | cargo run
```

Output:

```html
<span style="background-color:#7f007f; font-weight:700; font-style:italic; text-decoration:underline;">Styled text</span>
```

## macOS / iTerm2 Integration

`ansi2html` is particularly useful when integrated with iTerm2 and a Markdown editor that can render HTML elements.

The workflow uses iTerm2’s **Copy with Control Sequences** feature to preserve ANSI escape sequences, converts them to HTML, and places the resulting HTML on the clipboard so that you can paste it to your editor.

### 1\. Install the converter

Build the release binary and install it:

```shell
cargo build --release
mkdir -p ~/.local/bin
cp target/release/ansi2html ~/.local/bin/ansi2html
```

### 2\. Copy `a2h` wrapper

Copy `~/.local/bin/a2h`:

```shell
cp a2h ~/.local/bin/
```

This wrapper:
- Reads the copied terminal content from the clipboard.
- Converts ANSI escape sequences into HTML.
- Replaces empty lines with `&nbsp;`.
- Removes trailing spaces and tabs.
- Copies the resulting HTML to the clipboard.

### 3\. Create the iTerm2 AppleScript

Create the following script:

`~/Library/Application Support/iTerm2/Scripts/copy_as_html.scpt`

```applescript
tell application "System Events"
   tell process "iTerm2"
      click menu item "Copy with Control Sequences" of menu "Edit" of menu bar 1
      delay 0.2
      do shell script "~/.local/bin/a2h"
   end tell
end tell
```

This script:

1. Invokes iTerm2's **Copy with Control Sequences** menu item.
2. Waits briefly for the clipboard to be populated.
3. Runs `a2h` to convert the clipboard contents into HTML.

The delay is heuristic. If it didn’t work, please press **Command+Shift+C** again.

### 4\. Configure a keyboard shortcut

In iTerm2:

1. Open **Settings → Keys → Key Bindings**.
2. Add a new key binding.
3. Set the keyboard shortcut to **Shift+Command+C**.
4. Set the action to **Select Menu Item**.
5. Select `copy_as_html.scpt` as the menu item.

You can now select styled terminal output and press **Shift+Command+C** to copy it as HTML.

### 5\. Paste into HTML/Markdown

Open your HTML/Markdown editor, type `<pre><code></code></pre>`, and paste with **Command+V**.

The terminal output should retain its colors and text formatting.

**Note:** macOS may request accessibility permissions for iTerm2 to control its menu through System Events. Grant the necessary permission in System Settings → Privacy & Security → Automation.

Typing `<pre><code></code></pre>` repeatedly is cumbersome. You can use [espanso](https://espanso.org) with the following replacement rule:

```text
- trigger: ":tm"
  replace: '<pre><code>$|$</code></pre>'
```

## Supported ANSI Features

- Standard foreground colors (30–37)
- Standard background colors (40–47)
- Bright foreground colors (90–97)
- Bright background colors (100–107)
- 256-color foreground (38;5)
- 256-color background (48;5)
- Truecolor foreground (38;2)
- Truecolor background (48;2)
- Bold (1)
- Italic (3)
- Underline (4)
- Reverse video (7)
- Reset (0)

## Debug

```shell
RUST_LOG=debug cargo run
```

## License

MIT
