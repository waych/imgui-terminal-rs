#include "imgui-terminal-rs.h"

namespace terminal {

std::unique_ptr<Terminal> new_terminal() {
  return std::make_unique<Terminal>();
}

// Draw the terminal.
//
// TODO: Fix this to not require a rust::String to be passed in once imgui-rs
// figures out what replaces im_str!().
void draw_term(Terminal &terminal, rust::String str_id) {
  terminal.draw(str_id.c_str());
}

// Write bytes into the terminal.
void write_term(Terminal &terminal, rust::Str str) {
  terminal.write(str.data(), static_cast<int>(str.length()));
}

int32_t read_term(Terminal &terminal, rust::Slice<uint8_t> buffer) {
  return terminal.read(reinterpret_cast<char *>(buffer.data()),
                       static_cast<int>(buffer.length()));
}

} // namespace terminal
