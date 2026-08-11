Reporting Errors
==========================================================================

Good language tools are able to not only provide functionality, but report
errors if and when they occur, in a clear and informative manner. ``forj``
places priority on this; the user should be able to easily understand any
error, as well as where changes need to be made.

This is reflected in the :py:class:`Span` class, which accompanies many
objects (including :py:class:`Token`\ s, :py:class:`Node`\ s, and any
errors) and provides an exact location in the source files, so that
errors may be reported.

To report these errors, the :py:class:`Report` class can be used. This
class is created with a particular error location, but can have many labeled
sections of source code included in the error printout (see :py:attr:`Report.label`).
For example, take the following SystemVerilog code, referred to as ``test.v``:

.. code-block:: sv

   module test_module(;
   endmodule

Notice how we forgot the ending parentheses before the semicolon. Attempting
to :py:func:`parse` this will result in a :py:class:`ParserResult.ParserErr`;
the contained :py:class:`VerboseError` provides a :py:class:`Span` that could
be used to construct a :py:class:`Report`

.. code-block:: python

   from forj_python import parse, ParserResult, Report, ReportKind
   
   with open("test.v", "r") as f:
       content = f.read()

   parse_result = parse(content, "test.v", ["."], [])
   assert isinstance(parse_result, ParserResult.ParserErr)
   err_span = parse_result.error.span

   report = Report(ReportKind.Error(), err_span, "ERR", "Missing a )")
   report.label(err_span, ReportKind.Error(), "Put a ) here")
   report.print()

To display the content to a user, use the :py:meth:`Report.print` or :py:meth:`Report.eprint`
methods to print a :py:class:`Report` to ``stdout`` or ``stderr``,
respectively. Calling one of these on the above ``report`` produces
the following output

.. code-block:: text

   [ERR] Error: Missing a )
      ╭─[ test.v:1:20 ]
      │
    1 │ module test_module(;
      │                    ┬  
      │                    ╰── Put a ) here
   ───╯

For convenience, both :py:class:`VerboseError` and :py:class:`PreprocessorError`
also contain a ``report`` method, which produces a :py:class:`Report` suitable
for printing; one could instead modify the above code to use this directly

.. code-block:: python

   from forj_python import parse, ParserResult, Report, ReportKind
   
   with open("test.v", "r") as f:
       content = f.read()

   parse_result = parse(content, "test.v", ["."], [])
   assert isinstance(parse_result, ParserResult.ParserErr)
   report = parse_result.error.report()
   report.print()

which produces the following verbose but informative message

.. code-block:: text

   [P1] Error: found ;, expected ), ., an identifier, {, a comma, (, input, output, inout, ref, supply0, supply1, tri, triand, trior, trireg, tri0, tri1, uwire, wire, wand, wor, $unit, signed, unsigned, [, interface, bit, logic, reg, byte, shortint, int, longint, integer, time, shortreal, real, realtime, struct, union, enum, string, chandle, virtual, event, type, interconnect, or var
      ╭─[ test.v:1:20 ]
      │
    1 │ module test_module(;
      │                    ┬  
      │                    ╰── Didn't expect ;
   ───╯

Finally, if you ever find yourself using :py:class:`Span` whose :py:attr:`Span.file`
doesn't exist, use :py:meth:`Report.include` to include file content associated with
a particular name, so that a :py:class:`Report` can print appropriately