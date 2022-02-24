// Copyright 2021 pdfium-sys Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// Workaround for build problems on Windows. Removes access to FPDF_RenderPage.
#ifdef _WIN32
#undef _WIN32
#endif

// AJRC - 21/17/21 - adjusted from pdfium-sys version to include _all_ functions
// exposed by Pdfium, not just those in fpdfview.h.
#include "fpdf_annot.h"
#include "fpdf_attachment.h"
#include "fpdf_catalog.h"
#include "fpdf_dataavail.h"
#include "fpdf_doc.h"
#include "fpdf_edit.h"
#include "fpdf_ext.h"
#include "fpdf_flatten.h"
#include "fpdf_formfill.h"
#include "fpdf_fwlevent.h"
#include "fpdf_javascript.h"
#include "fpdf_ppo.h"
#include "fpdf_progressive.h"
#include "fpdf_save.h"
#include "fpdf_searchex.h"
#include "fpdf_signature.h"
#include "fpdf_structtree.h"
#include "fpdf_sysfontinfo.h"
#include "fpdf_text.h"
#include "fpdf_thumbnail.h"
#include "fpdf_transformpage.h"
#include "fpdfview.h"
