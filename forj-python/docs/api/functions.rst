Functions
==========================================================================

Lexing
--------------------------------------------------------------------------

.. py:function:: lex(src: str, file_name: str) -> list[SpannedToken]

   Separate a source file into syntactic tokens

   :param str src: The content of the source file to lex
   :param str file_name: The name of the source file, used to annotate :py:class:`Span`\s
   :return: The separated semantic tokens
   :rtype: list[SpannedToken]

Preprocessing
--------------------------------------------------------------------------

.. py:function:: preprocess(src: str, file_name: str, include_paths: Sequence[str | PathLike[str]], defines: Sequence[Define]) -> PreprocessorResult

   Preprocess a token stream, elaborating compiler directives

   :param str src: The content of the source file to preprocess
   :param str file_name: The name of the source file, used to annotate :py:class:`Span`\s
   :param include_paths: The include paths to search for include directives
   :type include_paths: Sequence[str | PathLike[str]]
   :param Sequence[Define] defines: Any initial preprocessor definitions to operate with
   :return: A result indicating either a successful preprocess or the resulting error
   :rtype: PreprocessorResult

.. py:function:: preprocess_from_lex(tokens: Sequence[SpannedToken], include_paths: Sequence[str | PathLike[str]], defines: Sequence[Define]) -> PreprocessorResult

   Same as :py:func:`preprocess`, but operates on the output of :py:func:`lex`
    
   Comparitively, this incurs overhead from copying data between
   Rust and Python's ownership models. Only use if you need to
   modify the output of :py:func:`lex` before preprocessing

   :param tokens: The tokens obtained (and possibly modified) from :py:func:`lex`
   :type tokens: Sequence[SpannedToken]
   :param include_paths: The include paths to search for include directives
   :type include_paths: Sequence[str | PathLike[str]]
   :param Sequence[Define] defines: Any initial preprocessor definitions to operate with
   :return: A result indicating either a successful preprocess or the resulting error
   :rtype: PreprocessorResult

Parsing
--------------------------------------------------------------------------

.. py:function:: parse(src: str, file_name: str, include_paths: Sequence[str | PathLike[str]], defines: Sequence[Define]) -> ParserResult

   Parse the token stream into a concrete syntax tree

   :param str src: The content of the source file to parse
   :param str file_name: The name of the source file, used to annotate :py:class:`Span`\s
   :type include_paths: Sequence[str | PathLike[str]]
   :param Sequence[Define] defines: Any initial preprocessor definitions to operate with
   :return: A result indicating either a successful parse or the resulting error
   :rtype: ParserResult

.. py:function:: parse_from_preprocess(tokens: Sequence[SpannedToken]) -> ParserResult

   Same as :py:func:`parse`, but operates on the output of :py:func:`preprocess`
    
   Comparitively, this incurs overhead from copying data between
   Rust and Python's ownership models. Only use if you need to
   modify the output of :py:func:`preprocess` before parsing

   :param tokens: The tokens obtained (and possibly modified) from :py:func:`preprocess` (or possibly :py:func:`lex`, if no preprocessing is required)
   :type tokens: Sequence[SpannedToken]
   :return: A result indicating either a successful parse or the resulting error
   :rtype: ParserResult

Utility
--------------------------------------------------------------------------

.. py:function:: define_empty(name: str) -> Define

   Create a :py:class:`Define` for a name with no replacement text

   :param str name: The name being defined
   :return: The preprocessor definition
   :rtype: Define

.. py:function:: define_text(name: str, text: str) -> Define

   Create a :py:class:`Define` for a name with some replacement text

   :param str name: The name being defined
   :param str text: The text to substitute for ``name`` when expanded
   :return: The preprocessor definition
   :rtype: Define
