Classes
==========================================================================

General
--------------------------------------------------------------------------

.. Token

.. py:class:: Bytes
   :final:
   
   A reference to a specific slice of a source file, with start and end
   byte offsets

   .. py:attribute:: start
      :type: int
   
      The byte-offset of the start of the slice (inclusive)
   
   .. py:attribute:: end
      :type: int
   
      The byte-offset of the end of the slice (exclusive)

.. py:class:: Define

   A preprocessor definition

   Users will likely want to use the helper functions
   :py:func:`define_empty` and :py:func:`define_text` to create :py:class:`Define`
   instances

   .. py:attribute:: name
      :type: str

      The name being defined

   .. py:attribute:: body
      :type: list[SpannedToken] | None

      The replacement tokens, if any, to use

.. py:class:: Expectation

   An expectation of what to find (instead of what was found while parsing)

   .. py:class:: Expectation.EOI(Expectation)
      :final:

      The end of a file

   .. py:class:: Expectation.Label(Expectation)
      :final:

      A verbose human-readable label

      .. py:property:: label
         :type: str

         The label text

   .. py:class:: Expectation.Token(Expectation)
      :final:

      A specific token

      .. py:property:: token
         :type: Token

         The expected token

.. py:class:: Node
   :final:

   A single CST node

   In addition to providing information about itself, :py:class:`Node`\s are also
   iterable (i.e. support the ``for node in node:`` syntax), which will produce
   both the original node, as well as all children nodes in the tree
   (recursively, depth-first)

   .. py:attribute:: name
      :type: str

      The name of the :py:class:`Node`

      See the corresponding `Rust documentation <https://docs.rs/forj-syntax/latest/forj_syntax/enum.Node.html>`_
      to see the possible names

   .. py:attribute:: span
      :type: Span

      The span of the :py:class:`Node`

   .. py:property:: text
      :type: str

      The text in the source code that the :py:class:`Node` corresponds to

      This is the same as getting :py:attr:`Span.text` from the :py:attr:`Node.span`

   .. py:attribute:: children
      :type: list[Node]

      All direct children of the :py:class:`Node` in the CST

.. py:class:: Span
   :final:

   Provides a location in the source code where a :py:class:`Node` was found

   .. py:attribute:: file
      :type: str
   
      The name of the file containing the source code
   
   .. py:attribute:: bytes
      :type: Bytes
   
      The byte-span of the :py:class:`Span` within the file

   .. py:property:: text
      :type: str

      The exact text in the source file that the :py:class:`Span` refers to
      (retrieved lazily for each access)

.. py:class:: SpannedToken
   :final:

   A source :py:class:`Token` with an associated :py:class:`Span`

   .. py:attribute:: token
      :type: Token

   .. py:attribute:: span
      :type: Span

.. py:class:: Token
   
   A single semantic token

   .. dropdown:: Child Variants

      .. py:class:: Token.AcceptOn(Token)

      .. py:class:: Token.Alias(Token)

      .. py:class:: Token.Always(Token)

      .. py:class:: Token.AlwaysComb(Token)

      .. py:class:: Token.AlwaysFf(Token)

      .. py:class:: Token.AlwaysLatch(Token)

      .. py:class:: Token.Amp(Token)

      .. py:class:: Token.AmpAmp(Token)

      .. py:class:: Token.AmpAmpAmp(Token)

      .. py:class:: Token.AmpEq(Token)

      .. py:class:: Token.And(Token)

      .. py:class:: Token.Apost(Token)

      .. py:class:: Token.Assert(Token)

      .. py:class:: Token.Assign(Token)

      .. py:class:: Token.Assume(Token)

      .. py:class:: Token.At(Token)

      .. py:class:: Token.AtAt(Token)

      .. py:class:: Token.Automatic(Token)

      .. py:class:: Token.Before(Token)

      .. py:class:: Token.Begin(Token)

      .. py:class:: Token.BinaryNumber(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.Bind(Token)

      .. py:class:: Token.Bins(Token)

      .. py:class:: Token.Binsof(Token)

      .. py:class:: Token.Bit(Token)

      .. py:class:: Token.BlockComment(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.Brace(Token)

      .. py:class:: Token.Bracket(Token)

      .. py:class:: Token.Break(Token)

      .. py:class:: Token.Bslash(Token)

      .. py:class:: Token.Buf(Token)

      .. py:class:: Token.Bufif0(Token)

      .. py:class:: Token.Bufif1(Token)

      .. py:class:: Token.Byte(Token)

      .. py:class:: Token.Caret(Token)

      .. py:class:: Token.CaretEq(Token)

      .. py:class:: Token.CaretTilde(Token)

      .. py:class:: Token.Case(Token)

      .. py:class:: Token.Casex(Token)

      .. py:class:: Token.Casez(Token)

      .. py:class:: Token.Cell(Token)

      .. py:class:: Token.Chandle(Token)

      .. py:class:: Token.Checker(Token)

      .. py:class:: Token.Class(Token)

      .. py:class:: Token.Clocking(Token)

      .. py:class:: Token.Cmos(Token)

      .. py:class:: Token.Colon(Token)

      .. py:class:: Token.ColonColon(Token)

      .. py:class:: Token.ColonEq(Token)

      .. py:class:: Token.ColonSlash(Token)

      .. py:class:: Token.Comma(Token)

      .. py:class:: Token.Config(Token)

      .. py:class:: Token.Const(Token)

      .. py:class:: Token.Constraint(Token)

      .. py:class:: Token.Context(Token)

      .. py:class:: Token.Continue(Token)

      .. py:class:: Token.Cover(Token)

      .. py:class:: Token.Covergroup(Token)

      .. py:class:: Token.Coverpoint(Token)

      .. py:class:: Token.Cross(Token)

      .. py:class:: Token.Deassign(Token)

      .. py:class:: Token.DecimalNumber(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.Default(Token)

      .. py:class:: Token.Defparam(Token)

      .. py:class:: Token.Design(Token)

      .. py:class:: Token.DirBeginKeywords(Token)

      .. py:class:: Token.DirCelldefine(Token)

      .. py:class:: Token.DirDefaultNettype(Token)

      .. py:class:: Token.DirDefine(Token)

      .. py:class:: Token.DirElse(Token)

      .. py:class:: Token.DirElsif(Token)

      .. py:class:: Token.DirEndKeywords(Token)

      .. py:class:: Token.DirEndcelldefine(Token)

      .. py:class:: Token.DirEndif(Token)

      .. py:class:: Token.DirIfdef(Token)

      .. py:class:: Token.DirIfndef(Token)

      .. py:class:: Token.DirInclude(Token)

      .. py:class:: Token.DirLine(Token)

      .. py:class:: Token.DirNounconnectedDrive(Token)

      .. py:class:: Token.DirPragma(Token)

      .. py:class:: Token.DirResetall(Token)

      .. py:class:: Token.DirTimescale(Token)

      .. py:class:: Token.DirUnconnectedDrive(Token)

      .. py:class:: Token.DirUndef(Token)

      .. py:class:: Token.DirUndefineall(Token)

      .. py:class:: Token.DirUnderscoreFile(Token)

      .. py:class:: Token.DirUnderscoreLine(Token)

      .. py:class:: Token.Disable(Token)

      .. py:class:: Token.Dist(Token)

      .. py:class:: Token.Do(Token)

      .. py:class:: Token.Dollar(Token)

      .. py:class:: Token.DollarError(Token)

      .. py:class:: Token.DollarFatal(Token)

      .. py:class:: Token.DollarFullskew(Token)

      .. py:class:: Token.DollarHold(Token)

      .. py:class:: Token.DollarInfo(Token)

      .. py:class:: Token.DollarNochange(Token)

      .. py:class:: Token.DollarPeriod(Token)

      .. py:class:: Token.DollarRecovery(Token)

      .. py:class:: Token.DollarRecrem(Token)

      .. py:class:: Token.DollarRemoval(Token)

      .. py:class:: Token.DollarRoot(Token)

      .. py:class:: Token.DollarSetup(Token)

      .. py:class:: Token.DollarSetuphold(Token)

      .. py:class:: Token.DollarSkew(Token)

      .. py:class:: Token.DollarTimeskew(Token)

      .. py:class:: Token.DollarUnit(Token)

      .. py:class:: Token.DollarWarning(Token)

      .. py:class:: Token.DollarWidth(Token)

      .. py:class:: Token.EBrace(Token)

      .. py:class:: Token.EBracket(Token)

      .. py:class:: Token.EParen(Token)

      .. py:class:: Token.Edge(Token)

      .. py:class:: Token.Else(Token)

      .. py:class:: Token.End(Token)

      .. py:class:: Token.Endcase(Token)

      .. py:class:: Token.Endchecker(Token)

      .. py:class:: Token.Endclass(Token)

      .. py:class:: Token.Endclocking(Token)

      .. py:class:: Token.Endconfig(Token)

      .. py:class:: Token.Endfunction(Token)

      .. py:class:: Token.Endgenerate(Token)

      .. py:class:: Token.Endgroup(Token)

      .. py:class:: Token.Endinterface(Token)

      .. py:class:: Token.Endmodule(Token)

      .. py:class:: Token.Endpackage(Token)

      .. py:class:: Token.Endprimitive(Token)

      .. py:class:: Token.Endprogram(Token)

      .. py:class:: Token.Endproperty(Token)

      .. py:class:: Token.Endsequence(Token)

      .. py:class:: Token.Endspecify(Token)

      .. py:class:: Token.Endtable(Token)

      .. py:class:: Token.Endtask(Token)

      .. py:class:: Token.Enum(Token)

      .. py:class:: Token.Eq(Token)

      .. py:class:: Token.EqEq(Token)

      .. py:class:: Token.EqEqEq(Token)

      .. py:class:: Token.EqEqQuest(Token)

      .. py:class:: Token.EqGt(Token)

      .. py:class:: Token.Error(Token)

         A lexer error (unrecognized input)

      .. py:class:: Token.EscapedIdentifier(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.Event(Token)

      .. py:class:: Token.Eventually(Token)

      .. py:class:: Token.ExclEq(Token)

      .. py:class:: Token.ExclEqEq(Token)

      .. py:class:: Token.ExclEqQuest(Token)

      .. py:class:: Token.Exclamation(Token)

      .. py:class:: Token.Expect(Token)

      .. py:class:: Token.Export(Token)

      .. py:class:: Token.Extends(Token)

      .. py:class:: Token.Extern(Token)

      .. py:class:: Token.Final(Token)

      .. py:class:: Token.FirstMatch(Token)

      .. py:class:: Token.FixedPointNumber(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.For(Token)

      .. py:class:: Token.Force(Token)

      .. py:class:: Token.Foreach(Token)

      .. py:class:: Token.Forever(Token)

      .. py:class:: Token.Fork(Token)

      .. py:class:: Token.Forkjoin(Token)

      .. py:class:: Token.Function(Token)

      .. py:class:: Token.Generate(Token)

      .. py:class:: Token.Genvar(Token)

      .. py:class:: Token.Global(Token)

      .. py:class:: Token.Gt(Token)

      .. py:class:: Token.GtEq(Token)

      .. py:class:: Token.GtGt(Token)

      .. py:class:: Token.GtGtEq(Token)

      .. py:class:: Token.GtGtGt(Token)

      .. py:class:: Token.GtGtGtEq(Token)

      .. py:class:: Token.HexNumber(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.Highz0(Token)

      .. py:class:: Token.Highz1(Token)

      .. py:class:: Token.If(Token)

      .. py:class:: Token.Iff(Token)

      .. py:class:: Token.Ifnone(Token)

      .. py:class:: Token.IgnoreBins(Token)

      .. py:class:: Token.IllegalBins(Token)

      .. py:class:: Token.Implements(Token)

      .. py:class:: Token.Implies(Token)

      .. py:class:: Token.Import(Token)

      .. py:class:: Token.Incdir(Token)

      .. py:class:: Token.Include(Token)

      .. py:class:: Token.Initial(Token)

      .. py:class:: Token.Inout(Token)

      .. py:class:: Token.Input(Token)

      .. py:class:: Token.Inside(Token)

      .. py:class:: Token.Instance(Token)

      .. py:class:: Token.Int(Token)

      .. py:class:: Token.Integer(Token)

      .. py:class:: Token.Interconnect(Token)

      .. py:class:: Token.Interface(Token)

      .. py:class:: Token.Intersect(Token)

      .. py:class:: Token.Join(Token)

      .. py:class:: Token.JoinAny(Token)

      .. py:class:: Token.JoinNone(Token)

      .. py:class:: Token.Large(Token)

      .. py:class:: Token.Let(Token)

      .. py:class:: Token.Liblist(Token)

      .. py:class:: Token.Library(Token)

      .. py:class:: Token.Local(Token)

      .. py:class:: Token.Localparam(Token)

      .. py:class:: Token.Logic(Token)

      .. py:class:: Token.Longint(Token)

      .. py:class:: Token.Lt(Token)

      .. py:class:: Token.LtEq(Token)

      .. py:class:: Token.LtLt(Token)

      .. py:class:: Token.LtLtEq(Token)

      .. py:class:: Token.LtLtLt(Token)

      .. py:class:: Token.LtLtLtEq(Token)

      .. py:class:: Token.LtMinusGt(Token)

      .. py:class:: Token.Macromodule(Token)

      .. py:class:: Token.Matches(Token)

      .. py:class:: Token.Medium(Token)

      .. py:class:: Token.Minus(Token)

      .. py:class:: Token.MinusColon(Token)

      .. py:class:: Token.MinusEq(Token)

      .. py:class:: Token.MinusGt(Token)

      .. py:class:: Token.MinusGtGt(Token)

      .. py:class:: Token.MinusMinus(Token)

      .. py:class:: Token.Modport(Token)

      .. py:class:: Token.Module(Token)

      .. py:class:: Token.Nand(Token)

      .. py:class:: Token.Negedge(Token)

      .. py:class:: Token.Nettype(Token)

      .. py:class:: Token.New(Token)

      .. py:class:: Token.Newline(Token)

      .. py:class:: Token.Nexttime(Token)

      .. py:class:: Token.Nmos(Token)

      .. py:class:: Token.Nor(Token)

      .. py:class:: Token.Noshowcancelled(Token)

      .. py:class:: Token.Not(Token)

      .. py:class:: Token.Notif0(Token)

      .. py:class:: Token.Notif1(Token)

      .. py:class:: Token.Null(Token)

      .. py:class:: Token.OctalNumber(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.OneStep(Token)

      .. py:class:: Token.OnelineComment(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.Or(Token)

      .. py:class:: Token.Output(Token)

      .. py:class:: Token.Package(Token)

      .. py:class:: Token.Packed(Token)

      .. py:class:: Token.Parameter(Token)

      .. py:class:: Token.Paren(Token)

      .. py:class:: Token.PathpulseDollar(Token)

      .. py:class:: Token.Percent(Token)

      .. py:class:: Token.PercentEq(Token)

      .. py:class:: Token.Period(Token)

      .. py:class:: Token.Pipe(Token)

      .. py:class:: Token.PipeEq(Token)

      .. py:class:: Token.PipeEqGt(Token)

      .. py:class:: Token.PipeMinusGt(Token)

      .. py:class:: Token.PipePipe(Token)

      .. py:class:: Token.Plus(Token)

      .. py:class:: Token.PlusColon(Token)

      .. py:class:: Token.PlusEq(Token)

      .. py:class:: Token.PlusPercentMinus(Token)

      .. py:class:: Token.PlusPlus(Token)

      .. py:class:: Token.PlusSlashMinus(Token)

      .. py:class:: Token.Pmos(Token)

      .. py:class:: Token.Posedge(Token)

      .. py:class:: Token.Pound(Token)

      .. py:class:: Token.PoundEqPound(Token)

      .. py:class:: Token.PoundMinusPound(Token)

      .. py:class:: Token.PoundPound(Token)

      .. py:class:: Token.PreprocessorIdentifier(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.PreprocessorStringLiteral(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.PreprocessorTripleQuoteStringLiteral(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.Primitive(Token)

      .. py:class:: Token.Priority(Token)

      .. py:class:: Token.Program(Token)

      .. py:class:: Token.Property(Token)

      .. py:class:: Token.Protected(Token)

      .. py:class:: Token.Pull0(Token)

      .. py:class:: Token.Pull1(Token)

      .. py:class:: Token.Pulldown(Token)

      .. py:class:: Token.Pullup(Token)

      .. py:class:: Token.PulsestyleOndetect(Token)

      .. py:class:: Token.PulsestyleOnevent(Token)

      .. py:class:: Token.Pure(Token)

      .. py:class:: Token.Quest(Token)

      .. py:class:: Token.Rand(Token)

      .. py:class:: Token.Randc(Token)

      .. py:class:: Token.Randcase(Token)

      .. py:class:: Token.Randsequence(Token)

      .. py:class:: Token.Rcmos(Token)

      .. py:class:: Token.Real(Token)

      .. py:class:: Token.Realtime(Token)

      .. py:class:: Token.Ref(Token)

      .. py:class:: Token.Reg(Token)

      .. py:class:: Token.RejectOn(Token)

      .. py:class:: Token.Release(Token)

      .. py:class:: Token.Repeat(Token)

      .. py:class:: Token.Restrict(Token)

      .. py:class:: Token.Return(Token)

      .. py:class:: Token.Rnmos(Token)

      .. py:class:: Token.Rpmos(Token)

      .. py:class:: Token.Rtran(Token)

      .. py:class:: Token.Rtranif0(Token)

      .. py:class:: Token.Rtranif1(Token)

      .. py:class:: Token.SAlways(Token)

      .. py:class:: Token.SColon(Token)

      .. py:class:: Token.SEventually(Token)

      .. py:class:: Token.SNexttime(Token)

      .. py:class:: Token.SUntil(Token)

      .. py:class:: Token.SUntilWith(Token)

      .. py:class:: Token.Scalared(Token)

      .. py:class:: Token.ScientificNumber(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.Sequence(Token)

      .. py:class:: Token.Shortint(Token)

      .. py:class:: Token.Shortreal(Token)

      .. py:class:: Token.Showcancelled(Token)

      .. py:class:: Token.Signed(Token)

      .. py:class:: Token.SimpleIdentifier(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.Slash(Token)

      .. py:class:: Token.SlashEq(Token)

      .. py:class:: Token.Small(Token)

      .. py:class:: Token.Soft(Token)

      .. py:class:: Token.Solve(Token)

      .. py:class:: Token.Specify(Token)

      .. py:class:: Token.Specparam(Token)

      .. py:class:: Token.Star(Token)

      .. py:class:: Token.StarEq(Token)

      .. py:class:: Token.StarGt(Token)

      .. py:class:: Token.StarStar(Token)

      .. py:class:: Token.Static(Token)

      .. py:class:: Token.String(Token)

      .. py:class:: Token.StringLiteral(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.Strong(Token)

      .. py:class:: Token.Strong0(Token)

      .. py:class:: Token.Strong1(Token)

      .. py:class:: Token.Struct(Token)

      .. py:class:: Token.Super(Token)

      .. py:class:: Token.Supply0(Token)

      .. py:class:: Token.Supply1(Token)

      .. py:class:: Token.SyncAcceptOn(Token)

      .. py:class:: Token.SyncRejectOn(Token)

      .. py:class:: Token.SystemTfIdentifier(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.Table(Token)

      .. py:class:: Token.Tagged(Token)

      .. py:class:: Token.Task(Token)

      .. py:class:: Token.TextMacro(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.ConcatenatedTextMacro(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.This(Token)

      .. py:class:: Token.Throughout(Token)

      .. py:class:: Token.Tilde(Token)

      .. py:class:: Token.TildeAmp(Token)

      .. py:class:: Token.TildeCaret(Token)

      .. py:class:: Token.TildePipe(Token)

      .. py:class:: Token.Time(Token)

      .. py:class:: Token.Timeprecision(Token)

      .. py:class:: Token.Timeunit(Token)

      .. py:class:: Token.Tran(Token)

      .. py:class:: Token.Tranif0(Token)

      .. py:class:: Token.Tranif1(Token)

      .. py:class:: Token.Tri(Token)

      .. py:class:: Token.Tri0(Token)

      .. py:class:: Token.Tri1(Token)

      .. py:class:: Token.Triand(Token)

      .. py:class:: Token.Trior(Token)

      .. py:class:: Token.TripleQuoteStringLiteral(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.Trireg(Token)

      .. py:class:: Token.Type(Token)

      .. py:class:: Token.Typedef(Token)

      .. py:class:: Token.UnbasedUnsizedLiteral(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.Union(Token)

      .. py:class:: Token.Unique(Token)

      .. py:class:: Token.Unique0(Token)

      .. py:class:: Token.Unsigned(Token)

      .. py:class:: Token.UnsignedNumber(Token)

         .. py:property:: text
            :type: str

            The text referenced by the token

      .. py:class:: Token.Until(Token)

      .. py:class:: Token.UntilWith(Token)

      .. py:class:: Token.Untyped(Token)

      .. py:class:: Token.Use(Token)

      .. py:class:: Token.Uwire(Token)

      .. py:class:: Token.Var(Token)

      .. py:class:: Token.Vectored(Token)

      .. py:class:: Token.Virtual(Token)

      .. py:class:: Token.Void(Token)

      .. py:class:: Token.Wait(Token)

      .. py:class:: Token.WaitOrder(Token)

      .. py:class:: Token.Wand(Token)

      .. py:class:: Token.Weak(Token)

      .. py:class:: Token.Weak0(Token)

      .. py:class:: Token.Weak1(Token)

      .. py:class:: Token.While(Token)

      .. py:class:: Token.Wildcard(Token)

      .. py:class:: Token.Wire(Token)

      .. py:class:: Token.With(Token)

      .. py:class:: Token.Within(Token)

      .. py:class:: Token.Wor(Token)

      .. py:class:: Token.Xnor(Token)

      .. py:class:: Token.Xor(Token)

.. py:class:: VerboseError
   :final:

   An error describing a :py:class:`Token` that was found and didn't match expectations

   .. py:attribute:: found
      :type: Token | None

      What token was found - ``None`` if the end of the file was reached
   
   .. py:attribute:: expected
      :type: list[Expectation]

      What was expected instead (listing all possibilities)

   .. py:attribute:: span
      :type: Span

      The :py:class:`Span` that the error occurred at (where the
      ``found`` token is)

Preprocessing
--------------------------------------------------------------------------

.. PreprocessorError, PreprocessorResult

.. py:class:: PreprocessorError

   An error that arose during preprocessing

   See the `Rust documentation <https://docs.rs/forj-parser/latest/forj_parser/preprocessor/enum.PreprocessorError.html>`_ for the semantics of each variant

   .. dropdown:: Child Variants

      .. py:class:: PreprocessorError.DuplicateMacroParameter(PreprocessorError)
         
         .. py:property:: define_name
            :type: str
   
               The name of the macro for which duplicate parameters were specified
         
         .. py:property:: dup_span
            :type: Span
   
               The :py:class:`Span` of the duplicate specification
         
         .. py:property:: param_name
            :type: str
   
               The name of the parameter that was specified multiple times
         
         .. py:property:: prev_span
            :type: Span
   
               The :py:class:`Span` of the previous/original specification
   
      .. py:class:: PreprocessorError.Else(PreprocessorError)
         
         .. py:property:: else_span
            :type: Span
   
               The :py:class:`Span` of the ``else``
   
      .. py:class:: PreprocessorError.Elsif(PreprocessorError)
         
         .. py:property:: elsif_span
            :type: Span
   
               The :py:class:`Span` of the ``elsif``
   
      .. py:class:: PreprocessorError.EndKeywords(PreprocessorError)
         
         .. py:property:: end_keywords_span
            :type: Span
   
               The :py:class:`Span` of the ``end_keywords``
   
      .. py:class:: PreprocessorError.Endif(PreprocessorError)
         
         .. py:property:: endif_span
            :type: Span
   
               The :py:class:`Span` of the ``endif``
   
      .. py:class:: PreprocessorError.Include(PreprocessorError)
         
         .. py:property:: include_path
            :type: str
   
               The path for the ``include`` directive
         
         .. py:property:: include_path_span
            :type: Span
   
               The :py:class:`Span` of the include path
         
         .. py:property:: read_err
            :type: str
   
               The I/O error raised when attempting to read the file
   
      .. py:class:: PreprocessorError.IncludeDepth(PreprocessorError)
         
         .. py:property:: include_span
            :type: Span
   
               The :py:class:`Span` of the ``include`` directive that exceeded the limit
   
      .. py:class:: PreprocessorError.IncompleteDefine(PreprocessorError)
         
         .. py:property:: other_span
            :type: Span
   
               The :py:class:`Span` of the token found instead
         
         .. py:property:: other_token
            :type: Token
   
               If known, the :py:class:`Token` found instead of a valid function macro argument specification
               
               In the case that the token wasn't tracked, the opening :py:class:`Token.Paren` is referenced
               instead
   
      .. py:class:: PreprocessorError.IncompleteDirective(PreprocessorError)
         
         .. py:property:: directive_span
            :type: Span
   
               The :py:class:`Span` of the incomplete preprocessor directive
               
               This is usually the primary directive, but can be other more indicative
               tokens as well, such as an unmatched opening parenthesis
   
      .. py:class:: PreprocessorError.IncompleteMacroWithToken(PreprocessorError)
         
         .. py:property:: error_span
            :type: Span
   
               The error-causing :py:class:`Span`
         
         .. py:property:: error_token
            :type: Token
   
               The error-causing :py:class:`Token` (either :py:class:`Token::EParen`\ ,
               :py:class:`Token::EBracket`\ , or :py:class:`Token::EBrace`\ )
   
      .. py:class:: PreprocessorError.RedefinedDirective(PreprocessorError)
         
         .. py:property:: directive_name
            :type: str
   
               The name of the compiler directive being redefined
         
         .. py:property:: directive_span
            :type: Span
   
               The :py:class:`Span` of the compiler directive name
      
      .. py:class:: PreprocessorError.InvalidDefineArgument(PreprocessorError)
         
         .. py:property:: other_span
            :type: Span
   
               The :py:class:`Span` of the token found instead
         
         .. py:property:: other_token
            :type: Token
   
               The :py:class:`Token` found instead of the valid ``define`` argument
   
      .. py:class:: PreprocessorError.InvalidDefineParameter(PreprocessorError)
         
         .. py:property:: other_span
            :type: Span
   
               The :py:class:`Span` of the token found instead
         
         .. py:property:: other_token
            :type: Token
   
               The :py:class:`Token` found instead of the ``define`` parameter
   
      .. py:class:: PreprocessorError.InvalidIdentifierFormation(PreprocessorError)
         
         .. py:property:: arg_span
            :type: Span
   
               The :py:class:`Span` of the invalid argument
         
         .. py:property:: param_name
            :type: str
   
               The name of the parameter used in a preprocessor identifier
   
      .. py:class:: PreprocessorError.InvalidRelativeTimescales(PreprocessorError)
         
         .. py:property:: timescale_span
            :type: Span
   
               The :py:class:`Span` of the ``timescale`` directive
   
      .. py:class:: PreprocessorError.InvalidVersionSpecifier(PreprocessorError)
         
         .. py:property:: invalid_version
            :type: Token
   
               The :py:class:`Token` provided instead of a valid version specifier
               
               If the token is a :py:class:`Token::StringLiteral`\ , the string isn't a version recognized
               by 1800-2023
         
         .. py:property:: invalid_version_span
            :type: Span
   
               The :py:class:`Span` of the invalid version specifier
   
      .. py:class:: PreprocessorError.MissingMacroArgument(PreprocessorError)
         
         .. py:property:: define_span
            :type: Span
   
               The :py:class:`Span` of the macro definition
         
         .. py:property:: param_name
            :type: str
   
               The name of the missing parameter
         
         .. py:property:: use_span
            :type: Span
   
               The :py:class:`Span` where the macro was used with a missing argument
   
      .. py:class:: PreprocessorError.NoDefaultAfterDefault(PreprocessorError)
         
         .. py:property:: default_param
            :type: str
   
               The name of the previously-specified default parameter
         
         .. py:property:: default_param_span
            :type: Span
   
               The :py:class:`Span` of the previously-specified default parameter
         
         .. py:property:: non_default_param
            :type: str
   
               The name of the non-default parameter
         
         .. py:property:: non_default_param_span
            :type: Span
   
               The :py:class:`Span` of the non-default parameter
   
      .. py:class:: PreprocessorError.NoEndKeywords(PreprocessorError)
         
         .. py:property:: begin_keywords_span
            :type: Span
   
               The :py:class:`Span` of the unterminated ``begin_keywords``
   
      .. py:class:: PreprocessorError.NoEndif(PreprocessorError)
         
         .. py:property:: cond_token
            :type: Token
   
               The conditional token (either :py:class:`Token::DirIfdef`\ , :py:class:`Token::DirIfndef`\ ,
               :py:class:`Token::DirElsif`\ , or :py:class:`Token::DirElse`\ ) with no matching `` `endif ``
         
         .. py:property:: cond_token_span
            :type: Span
   
               The :py:class:`Span` of the conditional token
   
      .. py:class:: PreprocessorError.NoMacroArguments(PreprocessorError)
         
         .. py:property:: define_span
            :type: Span
   
               The :py:class:`Span` of the macro definition (with arguments)
         
         .. py:property:: macro_name
            :type: str
   
               The name of the macro
         
         .. py:property:: use_span
            :type: Span
   
               The :py:class:`Span` where the macro was used with no arguments
   
      .. py:class:: PreprocessorError.NotPreviouslyDefinedMacro(PreprocessorError)
         
         .. py:property:: macro_name
            :type: str
   
               The name that wasn't previously defined
         
         .. py:property:: macro_span
            :type: Span
   
               The :py:class:`Span` where the not-previously-defined name was specified
   
      .. py:class:: PreprocessorError.RedefinedMacro(PreprocessorError)
         
         .. py:property:: macro_name
            :type: str
   
               The name of the macro being redefined
         
         .. py:property:: prev_def_span
            :type: Span
   
               The :py:class:`Span` where the macro was previously defined
         
         .. py:property:: redef_span
            :type: Span
   
               The :py:class:`Span` of the redefinition
   
      .. py:class:: PreprocessorError.TooManyMacroArguments(PreprocessorError)
         
         .. py:property:: define_span
            :type: Span
   
               The :py:class:`Span` of the macro definition
         
         .. py:property:: expected
            :type: int
   
               How many arguments were expected
         
         .. py:property:: found
            :type: int
   
               How many arguments were found
         
         .. py:property:: macro_name
            :type: str
   
               The name of the macro
         
         .. py:property:: use_span
            :type: Span
   
               The :py:class:`Span` where the macro was used with too many arguments
   
      .. py:class:: PreprocessorError.UndefinedMacro(PreprocessorError)
         
         .. py:property:: undefined_name
            :type: str
   
               The name of the undefined macro
         
         .. py:property:: undefined_span
            :type: Span
   
               The :py:class:`Span` of the undefined macro
   
      .. py:class:: PreprocessorError.VerboseError(PreprocessorError)
         
         .. py:property:: err
            :type: VerboseError
   
               The [`VerboseError`] for the preprocessor error

.. py:class:: PreprocessorResult

   The result of preprocessing a SystemVerilog source

   .. py:class:: PreprocessorResult.Ok(PreprocessorResult)
      :final:

      A successfully preprocessed token stream

      .. py:property:: tokens
         :type: list[SpannedToken]

   .. py:class:: PreprocessorResult.Err(PreprocessorResult)
      :final:

      Error(s) that occurred during preprocessing

      .. py:property:: errors
         :type: list[PreprocessorError]

Parsing
--------------------------------------------------------------------------

.. py:class:: ParserResult

   The result of parsing a SystemVerilog source

   .. py:class:: ParserResult.Ok(ParserResult)
      :final:

      A successfully parsed CST

      .. py:property:: root
         :type: Node

         The root of the CST (representing the ``SourceText`` :py:class:`Node`)

   .. py:class:: ParserResult.PreprocessorErr(ParserResult)
      :final:

      Error(s) that occurred during preprocessing

      .. py:property:: errors
         :type: list[PreprocessorError]

   .. py:class:: ParserResult.ParserErr(ParserResult)
      :final:

      An error that resulted during parsing

      .. py:property:: error
         :type: VerboseError

Reporting
--------------------------------------------------------------------------

.. py:class:: Report(kind: ReportKind, span: Span, code: String, msg: String)

   A report detailing an error when consuming a source file, including
   a location in that file.

   This does not label/print the location; to do so, use :py:meth:`Report.label`

   .. py:method:: eprint()

      Print the :py:class:`Report` to ``stdout``

   .. py:method:: include(file_name: str, file_content: str)

      Include additional files to use when printing.

      Use this when the :py:attr:`Span.file` used in a :py:class:`Report`
      does not correspond to an actual file on the file system
   
   .. py:method:: label(span: Span, kind: ReportKind, msg: str)

      Include a labeled section of source code in the :py:class:`Report`

   .. py:method:: print()

      Print the :py:class:`Report` to ``stdout``

.. py:class:: ReportKind

   The type of report being generated, used for coloring output

   .. py:class:: ReportKind.Advice(ReportKind)

   .. py:class:: ReportKind.Error(ReportKind)

   .. py:class:: ReportKind.Note(ReportKind)

   .. py:class:: ReportKind.Warning(ReportKind)