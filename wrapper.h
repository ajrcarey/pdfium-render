// Copyright 2021 pdfium-sys Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// Workaround for build problems on Windows.
/// Removes access to FPDF_RenderPage and FPDF_RenderPage.
#ifdef _WIN32
#undef _WIN32
#endif

#include "fpdfview.h"