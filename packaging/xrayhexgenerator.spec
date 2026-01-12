%define _name xrayhexgenerator
%define _version 2.2.2
%define _release 28
%define debug_package %{nil}

Name: %{_name}
Version: %{_version}
Release: %{_release}
Summary: HEX Generator
License: MIT
Group: Applications/Utilities
URL: https://github.com/XRayAdams/xrayhexgenerator-rs
BugURL: https://github.com/XRayAdams/xrayhexgenerator-rs/issues
Vendor: Konstantin Adamov

Source0: %{_name}-%{_version}.tar.gz
Source1: app.rayadams.xrayhexgenerator.desktop
Source2: app.rayadams.xrayhexgenerator.png
Source3: app.rayadams.xrayhexgenerator.metainfo.xml

Requires: gtk3, libstdc++

%description
A simple yet versatile HEX Generator application built with Rust + GTK4 for Linux desktops.

Features

Theme Support: Adapts to your system\'s light or dark theme settings for a native look and feel.

Variety of Generators: Provides several types of data generation:

Custom: Generate HEX strings with a user-defined number of digits.
GUID: Generate universally unique identifiers (UUID v4).
Mac Address: Generate random MAC addresses.
HEX Color: Generate random HEX color codes (e.g., #RRGGBB).
HEX Color with alpha: Generate random HEX color codes with an alpha channel (e.g., #AARRGGBB).
Byte Sequence: Generate sequences of HEX bytes (e.g., 00 FF 1A).
Prefixed HEX: Generate HEX strings with a "0x" prefix.
Customizable Output: Control the number of lines and digits (for applicable generators) and toggle uppercase output.
Easy Sharing & Saving: Copy generated data to the clipboard, share it, or save it to a file.

%prep
%setup -q -n release

%build
# This section is intentionally left blank as we are packaging a pre-compiled Flutter application.

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}/usr/bin
mkdir -p %{buildroot}/usr/share/applications
mkdir -p %{buildroot}/usr/share/icons/hicolor/256x256/apps
mkdir -p %{buildroot}/opt/%{_name}
mkdir -p %{buildroot}%{_datadir}/metainfo

# Copy the application files
cp -r ./* %{buildroot}/opt/%{_name}/

# Create a symlink in /usr/bin
ln -s /opt/%{_name}/%{_name} %{buildroot}/usr/bin/%{_name}

# Copy the desktop file
install -m 644 %{SOURCE1} %{buildroot}/usr/share/applications/%{_name}.desktop

# Copy the application icon
install -m 644 %{SOURCE2} %{buildroot}/usr/share/icons/hicolor/256x256/apps/%{_name}.png

# Copy meta info
install -m 644 %{SOURCE3} %{buildroot}%{_datadir}/metainfo/%{name}.metainfo.xml
%files
/usr/bin/%{_name}
/opt/%{_name}
/usr/share/applications/%{_name}.desktop
/usr/share/icons/hicolor/256x256/apps/%{_name}.png
%{_datadir}/metainfo/%{name}.metainfo.xml

%changelog
*loghere
- Initial RPM release
