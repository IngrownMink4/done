#!/bin/sh

rm -rf ../_build/
cd ..
meson setup _build .
meson dist -C _build/
cd _build/meson-dist/
nautilus .