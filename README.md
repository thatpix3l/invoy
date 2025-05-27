<h1 align="center">Freight Invoice Creator</h1>

<p align="center">
  <em>Simple - Native - Cross-platform</em>
</p>

# Purpose
A freighting company, of which I am employed at the time of writing, needed a way to quickly combine a trucking freight's bill of lading, its associated confirmation, and an internally made invoice summary into a single PDF file.

# Technologies
The the tool is written in flutter/dart for the UI, and Rust for the actual processing of data. For now, the tool simply spawns a shell process to call an already provided ghostscript binary.

# Directions for Utilization
Pick a directory containing the components of a final invoice:

<img src="assets/picking_invoice_dir.png">

Build into a single PDF file:

<img src="assets/created_invoice.png">
