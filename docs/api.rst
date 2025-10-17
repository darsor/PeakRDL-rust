.. _python_api:

Python API
==========
If you want to embed this tool into your own script, you can do so with the
following API.


Example
-------

The following example shows how to compile a SystemRDL file and then generate
the Rust code using the Python API.

.. code-block:: python

    from systemrdl import RDLCompiler
    from peakrdl_rust.exporter import RustExporter

    # compile the SystemRDL
    rdlc = RDLCompiler()
    rdlc.compile_file('example.rdl')
    top = rdlc.elaborate()

    # generate the Rust code
    exporter = RustExporter()
    exporter.export(node=top, path='registers')


Exporter Class
--------------

.. autoclass:: peakrdl_rust.exporter.RustExporter
    :members:

For more information about configuration options, see the :doc:`configuring` page.
