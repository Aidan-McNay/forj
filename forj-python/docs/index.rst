.. forj_python documentation master file, created by
   sphinx-quickstart on Mon Jun 22 22:55:29 2026.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

Python Bindings for Forj
==========================

To increase usability, `Forj <https://github.com/Aidan-McNay/forj>`_ provides
Python bindings to a limited subset of its functionality through the
`forj_python <https://pypi.org/project/forj_python/>`_ package. This allows
Python users to interact with SystemVerilog sources while leveraging the
existing (fast) tools written in Rust.

When crossing the FFI boundary, Rust's borrow checker can no longer
provide lifetime guarantees; as such, many data structures must be
cloned, and have associated ``From`` / ``Into`` implementations for
their reference-based Rust counterparts. This quickly becomes the
dominant factor in runtime; if speed/space usage becomes a concern,
native Rust applications should be considered instead.

.. warning::
    ``forj_python`` is currently considered unstable. Expect API breaks
    between minor versions.


.. toctree::
   :maxdepth: 2
   :caption: Overview

   parsing.rst
   reports.rst

.. toctree::
   :maxdepth: 2
   :caption: API Reference
   :hidden:

   api/functions.rst
   api/classes.rst

