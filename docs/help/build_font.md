Build font packages from source distributions.

Downloads font files from GitHub releases and packages them into
font-specific cbp archives for distribution.

Examples:
1. Build a font package:
   `cbp build font arial`

2. Build multiple font packages:
   `cbp build font arial courier`

3. Specify base directory:
   `cbp build font arial --base /path/to/project`