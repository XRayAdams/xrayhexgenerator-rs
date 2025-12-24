# HEX Generator

A simple yet versatile HEX Generator application built with Flutter for Linux desktops.

## Features

*   **Theme Support:** Adapts to your system's light or dark theme settings for a native look and feel.
*   **Variety of Generators:** Provides several types of data generation:
    *   **Custom:** Generate HEX strings with a user-defined number of digits.
    *   **GUID:** Generate universally unique identifiers (UUID v4).
    *   **Mac Address:** Generate random MAC addresses.
    *   **HEX Color:** Generate random HEX color codes (e.g., #RRGGBB).
    *   **HEX Color with alpha:** Generate random HEX color codes with an alpha channel (e.g., #AARRGGBB).
    *   **Byte Sequence:** Generate sequences of HEX bytes (e.g., 00 FF 1A).
    *   **Prefixed HEX:** Generate HEX strings with a "0x" prefix.
*   **Customizable Output:** Control the number of lines and digits (for applicable generators) and toggle uppercase output.
*   **Easy Sharing & Saving:** Copy generated data to the clipboard, share it, or save it to a file.

This project is open source and aims to provide a straightforward tool for developers and users who need to quickly generate various HEX-based data.

## Screenshots

<a href="screenshots/ss01.png"><img src="screenshots/ss01.png" alt="Screenshot 1" width="800"/></a>
<a href="screenshots/ss02.png"><img src="screenshots/ss02.png" alt="Screenshot 2" width="800"/></a>
<a href="screenshots/ss03.png"><img src="screenshots/ss03.png" alt="Screenshot 3" width="800"/></a>
<a href="screenshots/ss04.png"><img src="screenshots/ss04.png" alt="Screenshot 4" width="800"/></a>



## Installation sources
### From Snap Store

[![xrayhexgenerator](https://snapcraft.io/xrayhexgenerator/badge.svg)](https://snapcraft.io/xrayhexgenerator)

### As .dep package

1. Download the latest `.deb` package from the project's GitHub releases page.
2. Open a terminal and navigate to the directory where you downloaded the file.
3. Install the package using the following command:

   ```bash
   sudo dpkg -i [name-of-the-package].deb
   ```

### As .rpm package

1. Download the latest `.rpm` package from the project's GitHub releases page.
2. Open a terminal and navigate to the directory where you downloaded the file.
3. Install the package using the following command:

   ```bash
   sudo rpm -i [name-of-the-package].rpm
   ```
