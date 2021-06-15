#include <memory>
#include "terminal/Terminal.hh"
#include "rust/cxx.h"

namespace terminal {

std::unique_ptr<Terminal> new_terminal();
void draw_term(Terminal &terminal, rust::String str_id, bool &p_open);
void write_term(Terminal &terminal, rust::Str str);
int32_t read_term(Terminal &terminal, rust::Slice<uint8_t> buffer);

}  // namespace terminal
