

// https://dev.w3.org/html5/html-author/charref
// https://www.w3.org/TR/html5/entities.json
// https://en.wikipedia.org/wiki/List_of_XML_and_HTML_character_entity_references#Character_entity_references_in_HTML
// https://www.freeformatter.com/html-entities.html

// Syntax:
//      &#nnnn;
//      &#xhhhh;
//      &name;


pub static MAX_NAME_LENGTH: usize = 33;

pub static MINIMAL_ENTITIES: [(&'static str, char); 25] = [
    ("&AMP;", '&'),
    ("&GT;", '>'),
    ("&LT;", '<'),
    ("&MediumSpace;", '\u{8287}'),
    ("&NewLine;", '\n'),
    ("&NonBreakingSpace;", '\u{160}'),
    ("&QUOT;", '"'),
    ("&Tab;", '\u{9}'),
    ("&ThinSpace;", '\u{8201}'),
    ("&VeryThinSpace;", '\u{8202}'),
    ("&amp;", '&'),
    ("&apos;", '\''),
    ("&bsol;", '\\'),
    ("&emsp13;", '\u{8196}'),
    ("&emsp14;", '\u{8197}'),
    ("&emsp;", '\u{8195}'),
    ("&ensp;", '\u{8194}'),
    ("&gt;", '>'),
    ("&hairsp;", '\u{8202}'),
    ("&lt;", '<'),
    ("&nbsp;", '\u{160}'),
    ("&numsp;", '\u{8199}'),
    ("&puncsp;", '\u{8200}'),
    ("&quot;", '"'),
    ("&thinsp;", '\u{8201}'),
];

pub static NAMED_ENTITIES: [(&'static str, char); 2031] = [
    ("&AElig;", 'Æ'),
    ("&AMP;", '&'),
    ("&Aacute;", 'Á'),
    ("&Abreve;", 'Ă'),
    ("&Acirc;", 'Â'),
    ("&Acy;", 'А'),
    ("&Afr;", '𝔄'),
    ("&Agrave;", 'À'),
    ("&Alpha;", 'Α'),
    ("&Amacr;", 'Ā'),
    ("&And;", '⩓'),
    ("&Aogon;", 'Ą'),
    ("&Aopf;", '𝔸'),
    ("&ApplyFunction;", '⁡'),
    ("&Aring;", 'Å'),
    ("&Ascr;", '𝒜'),
    ("&Assign;", '≔'),
    ("&Atilde;", 'Ã'),
    ("&Auml;", 'Ä'),
    ("&Backslash;", '∖'),
    ("&Barv;", '⫧'),
    ("&Barwed;", '⌆'),
    ("&Bcy;", 'Б'),
    ("&Because;", '∵'),
    ("&Bernoullis;", 'ℬ'),
    ("&Beta;", 'Β'),
    ("&Bfr;", '𝔅'),
    ("&Bopf;", '𝔹'),
    ("&Breve;", '˘'),
    ("&Bscr;", 'ℬ'),
    ("&Bumpeq;", '≎'),
    ("&CHcy;", 'Ч'),
    ("&COPY;", '©'),
    ("&Cacute;", 'Ć'),
    ("&Cap;", '⋒'),
    ("&CapitalDifferentialD;", 'ⅅ'),
    ("&Cayleys;", 'ℭ'),
    ("&Ccaron;", 'Č'),
    ("&Ccedil;", 'Ç'),
    ("&Ccirc;", 'Ĉ'),
    ("&Cconint;", '∰'),
    ("&Cdot;", 'Ċ'),
    ("&Cedilla;", '¸'),
    ("&CenterDot;", '·'),
    ("&Cfr;", 'ℭ'),
    ("&Chi;", 'Χ'),
    ("&CircleDot;", '⊙'),
    ("&CircleMinus;", '⊖'),
    ("&CirclePlus;", '⊕'),
    ("&CircleTimes;", '⊗'),
    ("&ClockwiseContourIntegral;", '∲'),
    ("&CloseCurlyDoubleQuote;", '”'),
    ("&CloseCurlyQuote;", '’'),
    ("&Colon;", '∷'),
    ("&Colone;", '⩴'),
    ("&Congruent;", '≡'),
    ("&Conint;", '∯'),
    ("&ContourIntegral;", '∮'),
    ("&Copf;", 'ℂ'),
    ("&Coproduct;", '∐'),
    ("&CounterClockwiseContourIntegral;", '∳'),
    ("&Cross;", '⨯'),
    ("&Cscr;", '𝒞'),
    ("&Cup;", '⋓'),
    ("&CupCap;", '≍'),
    ("&DD;", 'ⅅ'),
    ("&DDotrahd;", '⤑'),
    ("&DJcy;", 'Ђ'),
    ("&DScy;", 'Ѕ'),
    ("&DZcy;", 'Џ'),
    ("&Dagger;", '‡'),
    ("&Darr;", '↡'),
    ("&Dashv;", '⫤'),
    ("&Dcaron;", 'Ď'),
    ("&Dcy;", 'Д'),
    ("&Del;", '∇'),
    ("&Delta;", 'Δ'),
    ("&Dfr;", '𝔇'),
    ("&DiacriticalAcute;", '´'),
    ("&DiacriticalDot;", '˙'),
    ("&DiacriticalDoubleAcute;", '˝'),
    ("&DiacriticalGrave;", '`'),
    ("&DiacriticalTilde;", '˜'),
    ("&Diamond;", '⋄'),
    ("&DifferentialD;", 'ⅆ'),
    ("&Dopf;", '𝔻'),
    ("&Dot;", '¨'),
    ("&DotDot;", '⃜'),
    ("&DotEqual;", '≐'),
    ("&DoubleContourIntegral;", '∯'),
    ("&DoubleDot;", '¨'),
    ("&DoubleDownArrow;", '⇓'),
    ("&DoubleLeftArrow;", '⇐'),
    ("&DoubleLeftRightArrow;", '⇔'),
    ("&DoubleLeftTee;", '⫤'),
    ("&DoubleLongLeftArrow;", '⟸'),
    ("&DoubleLongLeftRightArrow;", '⟺'),
    ("&DoubleLongRightArrow;", '⟹'),
    ("&DoubleRightArrow;", '⇒'),
    ("&DoubleRightTee;", '⊨'),
    ("&DoubleUpArrow;", '⇑'),
    ("&DoubleUpDownArrow;", '⇕'),
    ("&DoubleVerticalBar;", '∥'),
    ("&DownArrow;", '↓'),
    ("&DownArrowBar;", '⤓'),
    ("&DownArrowUpArrow;", '⇵'),
    ("&DownBreve;", '̑'),
    ("&DownLeftRightVector;", '⥐'),
    ("&DownLeftTeeVector;", '⥞'),
    ("&DownLeftVector;", '↽'),
    ("&DownLeftVectorBar;", '⥖'),
    ("&DownRightTeeVector;", '⥟'),
    ("&DownRightVector;", '⇁'),
    ("&DownRightVectorBar;", '⥗'),
    ("&DownTee;", '⊤'),
    ("&DownTeeArrow;", '↧'),
    ("&Downarrow;", '⇓'),
    ("&Dscr;", '𝒟'),
    ("&Dstrok;", 'Đ'),
    ("&ENG;", 'Ŋ'),
    ("&ETH;", 'Ð'),
    ("&Eacute;", 'É'),
    ("&Ecaron;", 'Ě'),
    ("&Ecirc;", 'Ê'),
    ("&Ecy;", 'Э'),
    ("&Edot;", 'Ė'),
    ("&Efr;", '𝔈'),
    ("&Egrave;", 'È'),
    ("&Element;", '∈'),
    ("&Emacr;", 'Ē'),
    ("&EmptySmallSquare;", '◻'),
    ("&EmptyVerySmallSquare;", '▫'),
    ("&Eogon;", 'Ę'),
    ("&Eopf;", '𝔼'),
    ("&Epsilon;", 'Ε'),
    ("&Equal;", '⩵'),
    ("&EqualTilde;", '≂'),
    ("&Equilibrium;", '⇌'),
    ("&Escr;", 'ℰ'),
    ("&Esim;", '⩳'),
    ("&Eta;", 'Η'),
    ("&Euml;", 'Ë'),
    ("&Exists;", '∃'),
    ("&ExponentialE;", 'ⅇ'),
    ("&Fcy;", 'Ф'),
    ("&Ffr;", '𝔉'),
    ("&FilledSmallSquare;", '◼'),
    ("&FilledVerySmallSquare;", '▪'),
    ("&Fopf;", '𝔽'),
    ("&ForAll;", '∀'),
    ("&Fouriertrf;", 'ℱ'),
    ("&Fscr;", 'ℱ'),
    ("&GJcy;", 'Ѓ'),
    ("&GT;", '>'),
    ("&Gamma;", 'Γ'),
    ("&Gammad;", 'Ϝ'),
    ("&Gbreve;", 'Ğ'),
    ("&Gcedil;", 'Ģ'),
    ("&Gcirc;", 'Ĝ'),
    ("&Gcy;", 'Г'),
    ("&Gdot;", 'Ġ'),
    ("&Gfr;", '𝔊'),
    ("&Gg;", '⋙'),
    ("&Gopf;", '𝔾'),
    ("&GreaterEqual;", '≥'),
    ("&GreaterEqualLess;", '⋛'),
    ("&GreaterFullEqual;", '≧'),
    ("&GreaterGreater;", '⪢'),
    ("&GreaterLess;", '≷'),
    ("&GreaterSlantEqual;", '⩾'),
    ("&GreaterTilde;", '≳'),
    ("&Gscr;", '𝒢'),
    ("&Gt;", '≫'),
    ("&HARDcy;", 'Ъ'),
    ("&Hacek;", 'ˇ'),
    ("&Hat;", '^'),
    ("&Hcirc;", 'Ĥ'),
    ("&Hfr;", 'ℌ'),
    ("&HilbertSpace;", 'ℋ'),
    ("&Hopf;", 'ℍ'),
    ("&HorizontalLine;", '─'),
    ("&Hscr;", 'ℋ'),
    ("&Hstrok;", 'Ħ'),
    ("&HumpDownHump;", '≎'),
    ("&HumpEqual;", '≏'),
    ("&IEcy;", 'Е'),
    ("&IJlig;", 'Ĳ'),
    ("&IOcy;", 'Ё'),
    ("&Iacute;", 'Í'),
    ("&Icirc;", 'Î'),
    ("&Icy;", 'И'),
    ("&Idot;", 'İ'),
    ("&Ifr;", 'ℑ'),
    ("&Igrave;", 'Ì'),
    ("&Im;", 'ℑ'),
    ("&Imacr;", 'Ī'),
    ("&ImaginaryI;", 'ⅈ'),
    ("&Implies;", '⇒'),
    ("&Int;", '∬'),
    ("&Integral;", '∫'),
    ("&Intersection;", '⋂'),
    ("&InvisibleComma;", '⁣'),
    ("&InvisibleTimes;", '⁢'),
    ("&Iogon;", 'Į'),
    ("&Iopf;", '𝕀'),
    ("&Iota;", 'Ι'),
    ("&Iscr;", 'ℐ'),
    ("&Itilde;", 'Ĩ'),
    ("&Iukcy;", 'І'),
    ("&Iuml;", 'Ï'),
    ("&Jcirc;", 'Ĵ'),
    ("&Jcy;", 'Й'),
    ("&Jfr;", '𝔍'),
    ("&Jopf;", '𝕁'),
    ("&Jscr;", '𝒥'),
    ("&Jsercy;", 'Ј'),
    ("&Jukcy;", 'Є'),
    ("&KHcy;", 'Х'),
    ("&KJcy;", 'Ќ'),
    ("&Kappa;", 'Κ'),
    ("&Kcedil;", 'Ķ'),
    ("&Kcy;", 'К'),
    ("&Kfr;", '𝔎'),
    ("&Kopf;", '𝕂'),
    ("&Kscr;", '𝒦'),
    ("&LJcy;", 'Љ'),
    ("&LT;", '<'),
    ("&Lacute;", 'Ĺ'),
    ("&Lambda;", 'Λ'),
    ("&Lang;", '⟪'),
    ("&Laplacetrf;", 'ℒ'),
    ("&Larr;", '↞'),
    ("&Lcaron;", 'Ľ'),
    ("&Lcedil;", 'Ļ'),
    ("&Lcy;", 'Л'),
    ("&LeftAngleBracket;", '⟨'),
    ("&LeftArrow;", '←'),
    ("&LeftArrowBar;", '⇤'),
    ("&LeftArrowRightArrow;", '⇆'),
    ("&LeftCeiling;", '⌈'),
    ("&LeftDoubleBracket;", '⟦'),
    ("&LeftDownTeeVector;", '⥡'),
    ("&LeftDownVector;", '⇃'),
    ("&LeftDownVectorBar;", '⥙'),
    ("&LeftFloor;", '⌊'),
    ("&LeftRightArrow;", '↔'),
    ("&LeftRightVector;", '⥎'),
    ("&LeftTee;", '⊣'),
    ("&LeftTeeArrow;", '↤'),
    ("&LeftTeeVector;", '⥚'),
    ("&LeftTriangle;", '⊲'),
    ("&LeftTriangleBar;", '⧏'),
    ("&LeftTriangleEqual;", '⊴'),
    ("&LeftUpDownVector;", '⥑'),
    ("&LeftUpTeeVector;", '⥠'),
    ("&LeftUpVector;", '↿'),
    ("&LeftUpVectorBar;", '⥘'),
    ("&LeftVector;", '↼'),
    ("&LeftVectorBar;", '⥒'),
    ("&Leftarrow;", '⇐'),
    ("&Leftrightarrow;", '⇔'),
    ("&LessEqualGreater;", '⋚'),
    ("&LessFullEqual;", '≦'),
    ("&LessGreater;", '≶'),
    ("&LessLess;", '⪡'),
    ("&LessSlantEqual;", '⩽'),
    ("&LessTilde;", '≲'),
    ("&Lfr;", '𝔏'),
    ("&Ll;", '⋘'),
    ("&Lleftarrow;", '⇚'),
    ("&Lmidot;", 'Ŀ'),
    ("&LongLeftArrow;", '⟵'),
    ("&LongLeftRightArrow;", '⟷'),
    ("&LongRightArrow;", '⟶'),
    ("&Longleftarrow;", '⟸'),
    ("&Longleftrightarrow;", '⟺'),
    ("&Longrightarrow;", '⟹'),
    ("&Lopf;", '𝕃'),
    ("&LowerLeftArrow;", '↙'),
    ("&LowerRightArrow;", '↘'),
    ("&Lscr;", 'ℒ'),
    ("&Lsh;", '↰'),
    ("&Lstrok;", 'Ł'),
    ("&Lt;", '≪'),
    ("&Map;", '⤅'),
    ("&Mcy;", 'М'),
    ("&MediumSpace;", '\u{8287}'),
    ("&Mellintrf;", 'ℳ'),
    ("&Mfr;", '𝔐'),
    ("&MinusPlus;", '∓'),
    ("&Mopf;", '𝕄'),
    ("&Mscr;", 'ℳ'),
    ("&Mu;", 'Μ'),
    ("&NJcy;", 'Њ'),
    ("&Nacute;", 'Ń'),
    ("&Ncaron;", 'Ň'),
    ("&Ncedil;", 'Ņ'),
    ("&Ncy;", 'Н'),
    ("&NegativeMediumSpace;", '​'),
    ("&NegativeThickSpace;", '​'),
    ("&NegativeThinSpace;", '​'),
    ("&NegativeVeryThinSpace;", '​'),
    ("&NestedGreaterGreater;", '≫'),
    ("&NestedLessLess;", '≪'),
    ("&NewLine;", '\n'),
    ("&Nfr;", '𝔑'),
    ("&NoBreak;", '⁠'),
    ("&NonBreakingSpace;", '\u{160}'),
    ("&Nopf;", 'ℕ'),
    ("&Not;", '⫬'),
    ("&NotCongruent;", '≢'),
    ("&NotCupCap;", '≭'),
    ("&NotDoubleVerticalBar;", '∦'),
    ("&NotElement;", '∉'),
    ("&NotEqual;", '≠'),
    ("&NotExists;", '∄'),
    ("&NotGreater;", '≯'),
    ("&NotGreaterEqual;", '≱'),
    ("&NotGreaterLess;", '≹'),
    ("&NotGreaterTilde;", '≵'),
    ("&NotLeftTriangle;", '⋪'),
    ("&NotLeftTriangleEqual;", '⋬'),
    ("&NotLess;", '≮'),
    ("&NotLessEqual;", '≰'),
    ("&NotLessGreater;", '≸'),
    ("&NotLessTilde;", '≴'),
    ("&NotPrecedes;", '⊀'),
    ("&NotPrecedesSlantEqual;", '⋠'),
    ("&NotReverseElement;", '∌'),
    ("&NotRightTriangle;", '⋫'),
    ("&NotRightTriangleEqual;", '⋭'),
    ("&NotSquareSubsetEqual;", '⋢'),
    ("&NotSquareSupersetEqual;", '⋣'),
    ("&NotSubsetEqual;", '⊈'),
    ("&NotSucceeds;", '⊁'),
    ("&NotSucceedsSlantEqual;", '⋡'),
    ("&NotSupersetEqual;", '⊉'),
    ("&NotTilde;", '≁'),
    ("&NotTildeEqual;", '≄'),
    ("&NotTildeFullEqual;", '≇'),
    ("&NotTildeTilde;", '≉'),
    ("&NotVerticalBar;", '∤'),
    ("&Nscr;", '𝒩'),
    ("&Ntilde;", 'Ñ'),
    ("&Nu;", 'Ν'),
    ("&OElig;", 'Œ'),
    ("&Oacute;", 'Ó'),
    ("&Ocirc;", 'Ô'),
    ("&Ocy;", 'О'),
    ("&Odblac;", 'Ő'),
    ("&Ofr;", '𝔒'),
    ("&Ograve;", 'Ò'),
    ("&Omacr;", 'Ō'),
    ("&Omega;", 'Ω'),
    ("&Omicron;", 'Ο'),
    ("&Oopf;", '𝕆'),
    ("&OpenCurlyDoubleQuote;", '“'),
    ("&OpenCurlyQuote;", '‘'),
    ("&Or;", '⩔'),
    ("&Oscr;", '𝒪'),
    ("&Oslash;", 'Ø'),
    ("&Otilde;", 'Õ'),
    ("&Otimes;", '⨷'),
    ("&Ouml;", 'Ö'),
    ("&OverBar;", '¯'),
    ("&OverBrace;", '⏞'),
    ("&OverBracket;", '⎴'),
    ("&OverParenthesis;", '⏜'),
    ("&PartialD;", '∂'),
    ("&Pcy;", 'П'),
    ("&Pfr;", '𝔓'),
    ("&Phi;", 'Φ'),
    ("&Pi;", 'Π'),
    ("&PlusMinus;", '±'),
    ("&Poincareplane;", 'ℌ'),
    ("&Popf;", 'ℙ'),
    ("&Pr;", '⪻'),
    ("&Precedes;", '≺'),
    ("&PrecedesEqual;", '⪯'),
    ("&PrecedesSlantEqual;", '≼'),
    ("&PrecedesTilde;", '≾'),
    ("&Prime;", '″'),
    ("&Product;", '∏'),
    ("&Proportion;", '∷'),
    ("&Proportional;", '∝'),
    ("&Pscr;", '𝒫'),
    ("&Psi;", 'Ψ'),
    ("&QUOT;", '"'),
    ("&Qfr;", '𝔔'),
    ("&Qopf;", 'ℚ'),
    ("&Qscr;", '𝒬'),
    ("&RBarr;", '⤐'),
    ("&REG;", '®'),
    ("&Racute;", 'Ŕ'),
    ("&Rang;", '⟫'),
    ("&Rarr;", '↠'),
    ("&Rarrtl;", '⤖'),
    ("&Rcaron;", 'Ř'),
    ("&Rcedil;", 'Ŗ'),
    ("&Rcy;", 'Р'),
    ("&Re;", 'ℜ'),
    ("&ReverseElement;", '∋'),
    ("&ReverseEquilibrium;", '⇋'),
    ("&ReverseUpEquilibrium;", '⥯'),
    ("&Rfr;", 'ℜ'),
    ("&Rho;", 'Ρ'),
    ("&RightAngleBracket;", '⟩'),
    ("&RightArrow;", '→'),
    ("&RightArrowBar;", '⇥'),
    ("&RightArrowLeftArrow;", '⇄'),
    ("&RightCeiling;", '⌉'),
    ("&RightDoubleBracket;", '⟧'),
    ("&RightDownTeeVector;", '⥝'),
    ("&RightDownVector;", '⇂'),
    ("&RightDownVectorBar;", '⥕'),
    ("&RightFloor;", '⌋'),
    ("&RightTee;", '⊢'),
    ("&RightTeeArrow;", '↦'),
    ("&RightTeeVector;", '⥛'),
    ("&RightTriangle;", '⊳'),
    ("&RightTriangleBar;", '⧐'),
    ("&RightTriangleEqual;", '⊵'),
    ("&RightUpDownVector;", '⥏'),
    ("&RightUpTeeVector;", '⥜'),
    ("&RightUpVector;", '↾'),
    ("&RightUpVectorBar;", '⥔'),
    ("&RightVector;", '⇀'),
    ("&RightVectorBar;", '⥓'),
    ("&Rightarrow;", '⇒'),
    ("&Ropf;", 'ℝ'),
    ("&RoundImplies;", '⥰'),
    ("&Rrightarrow;", '⇛'),
    ("&Rscr;", 'ℛ'),
    ("&Rsh;", '↱'),
    ("&RuleDelayed;", '⧴'),
    ("&SHCHcy;", 'Щ'),
    ("&SHcy;", 'Ш'),
    ("&SOFTcy;", 'Ь'),
    ("&Sacute;", 'Ś'),
    ("&Sc;", '⪼'),
    ("&Scaron;", 'Š'),
    ("&Scedil;", 'Ş'),
    ("&Scirc;", 'Ŝ'),
    ("&Scy;", 'С'),
    ("&Sfr;", '𝔖'),
    ("&ShortDownArrow;", '↓'),
    ("&ShortLeftArrow;", '←'),
    ("&ShortRightArrow;", '→'),
    ("&ShortUpArrow;", '↑'),
    ("&Sigma;", 'Σ'),
    ("&SmallCircle;", '∘'),
    ("&Sopf;", '𝕊'),
    ("&Sqrt;", '√'),
    ("&Square;", '□'),
    ("&SquareIntersection;", '⊓'),
    ("&SquareSubset;", '⊏'),
    ("&SquareSubsetEqual;", '⊑'),
    ("&SquareSuperset;", '⊐'),
    ("&SquareSupersetEqual;", '⊒'),
    ("&SquareUnion;", '⊔'),
    ("&Sscr;", '𝒮'),
    ("&Star;", '⋆'),
    ("&Sub;", '⋐'),
    ("&Subset;", '⋐'),
    ("&SubsetEqual;", '⊆'),
    ("&Succeeds;", '≻'),
    ("&SucceedsEqual;", '⪰'),
    ("&SucceedsSlantEqual;", '≽'),
    ("&SucceedsTilde;", '≿'),
    ("&SuchThat;", '∋'),
    ("&Sum;", '∑'),
    ("&Sup;", '⋑'),
    ("&Superset;", '⊃'),
    ("&SupersetEqual;", '⊇'),
    ("&Supset;", '⋑'),
    ("&THORN;", 'Þ'),
    ("&TRADE;", '™'),
    ("&TSHcy;", 'Ћ'),
    ("&TScy;", 'Ц'),
    ("&Tab;", '\u{9}'),
    ("&Tau;", 'Τ'),
    ("&Tcaron;", 'Ť'),
    ("&Tcedil;", 'Ţ'),
    ("&Tcy;", 'Т'),
    ("&Tfr;", '𝔗'),
    ("&Therefore;", '∴'),
    ("&Theta;", 'Θ'),
    ("&ThinSpace;", '\u{8201}'),
    ("&Tilde;", '∼'),
    ("&TildeEqual;", '≃'),
    ("&TildeFullEqual;", '≅'),
    ("&TildeTilde;", '≈'),
    ("&Topf;", '𝕋'),
    ("&TripleDot;", '⃛'),
    ("&Tscr;", '𝒯'),
    ("&Tstrok;", 'Ŧ'),
    ("&Uacute;", 'Ú'),
    ("&Uarr;", '↟'),
    ("&Uarrocir;", '⥉'),
    ("&Ubrcy;", 'Ў'),
    ("&Ubreve;", 'Ŭ'),
    ("&Ucirc;", 'Û'),
    ("&Ucy;", 'У'),
    ("&Udblac;", 'Ű'),
    ("&Ufr;", '𝔘'),
    ("&Ugrave;", 'Ù'),
    ("&Umacr;", 'Ū'),
    ("&UnderBar;", '̲'),
    ("&UnderBrace;", '⏟'),
    ("&UnderBracket;", '⎵'),
    ("&UnderParenthesis;", '⏝'),
    ("&Union;", '⋃'),
    ("&UnionPlus;", '⊎'),
    ("&Uogon;", 'Ų'),
    ("&Uopf;", '𝕌'),
    ("&UpArrow;", '↑'),
    ("&UpArrowBar;", '⤒'),
    ("&UpArrowDownArrow;", '⇅'),
    ("&UpDownArrow;", '↕'),
    ("&UpEquilibrium;", '⥮'),
    ("&UpTee;", '⊥'),
    ("&UpTeeArrow;", '↥'),
    ("&Uparrow;", '⇑'),
    ("&Updownarrow;", '⇕'),
    ("&UpperLeftArrow;", '↖'),
    ("&UpperRightArrow;", '↗'),
    ("&Upsi;", 'ϒ'),
    ("&Upsilon;", 'Υ'),
    ("&Uring;", 'Ů'),
    ("&Uscr;", '𝒰'),
    ("&Utilde;", 'Ũ'),
    ("&Uuml;", 'Ü'),
    ("&VDash;", '⊫'),
    ("&Vbar;", '⫫'),
    ("&Vcy;", 'В'),
    ("&Vdash;", '⊩'),
    ("&Vdashl;", '⫦'),
    ("&Vee;", '⋁'),
    ("&Verbar;", '‖'),
    ("&Vert;", '‖'),
    ("&VerticalBar;", '∣'),
    ("&VerticalLine;", '|'),
    ("&VerticalSeparator;", '❘'),
    ("&VerticalTilde;", '≀'),
    ("&VeryThinSpace;", '\u{8202}'),
    ("&Vfr;", '𝔙'),
    ("&Vopf;", '𝕍'),
    ("&Vscr;", '𝒱'),
    ("&Vvdash;", '⊪'),
    ("&Wcirc;", 'Ŵ'),
    ("&Wedge;", '⋀'),
    ("&Wfr;", '𝔚'),
    ("&Wopf;", '𝕎'),
    ("&Wscr;", '𝒲'),
    ("&Xfr;", '𝔛'),
    ("&Xi;", 'Ξ'),
    ("&Xopf;", '𝕏'),
    ("&Xscr;", '𝒳'),
    ("&YAcy;", 'Я'),
    ("&YIcy;", 'Ї'),
    ("&YUcy;", 'Ю'),
    ("&Yacute;", 'Ý'),
    ("&Ycirc;", 'Ŷ'),
    ("&Ycy;", 'Ы'),
    ("&Yfr;", '𝔜'),
    ("&Yopf;", '𝕐'),
    ("&Yscr;", '𝒴'),
    ("&Yuml;", 'Ÿ'),
    ("&ZHcy;", 'Ж'),
    ("&Zacute;", 'Ź'),
    ("&Zcaron;", 'Ž'),
    ("&Zcy;", 'З'),
    ("&Zdot;", 'Ż'),
    ("&ZeroWidthSpace;", '​'),
    ("&Zeta;", 'Ζ'),
    ("&Zfr;", 'ℨ'),
    ("&Zopf;", 'ℤ'),
    ("&Zscr;", '𝒵'),
    ("&aacute;", 'á'),
    ("&abreve;", 'ă'),
    ("&ac;", '∾'),
    ("&acd;", '∿'),
    ("&acirc;", 'â'),
    ("&acute;", '´'),
    ("&acy;", 'а'),
    ("&aelig;", 'æ'),
    ("&af;", '⁡'),
    ("&afr;", '𝔞'),
    ("&agrave;", 'à'),
    ("&alefsym;", 'ℵ'),
    ("&aleph;", 'ℵ'),
    ("&alpha;", 'α'),
    ("&amacr;", 'ā'),
    ("&amalg;", '⨿'),
    ("&amp;", '&'),
    ("&and;", '∧'),
    ("&andand;", '⩕'),
    ("&andd;", '⩜'),
    ("&andslope;", '⩘'),
    ("&andv;", '⩚'),
    ("&ang;", '∠'),
    ("&ange;", '⦤'),
    ("&angle;", '∠'),
    ("&angmsd;", '∡'),
    ("&angmsdaa;", '⦨'),
    ("&angmsdab;", '⦩'),
    ("&angmsdac;", '⦪'),
    ("&angmsdad;", '⦫'),
    ("&angmsdae;", '⦬'),
    ("&angmsdaf;", '⦭'),
    ("&angmsdag;", '⦮'),
    ("&angmsdah;", '⦯'),
    ("&angrt;", '∟'),
    ("&angrtvb;", '⊾'),
    ("&angrtvbd;", '⦝'),
    ("&angsph;", '∢'),
    ("&angst;", 'Å'),
    ("&angzarr;", '⍼'),
    ("&aogon;", 'ą'),
    ("&aopf;", '𝕒'),
    ("&ap;", '≈'),
    ("&apE;", '⩰'),
    ("&apacir;", '⩯'),
    ("&ape;", '≊'),
    ("&apid;", '≋'),
    ("&apos;", '\''),
    ("&approx;", '≈'),
    ("&approxeq;", '≊'),
    ("&aring;", 'å'),
    ("&ascr;", '𝒶'),
    ("&ast;", '*'),
    ("&asymp;", '≈'),
    ("&asympeq;", '≍'),
    ("&atilde;", 'ã'),
    ("&auml;", 'ä'),
    ("&awconint;", '∳'),
    ("&awint;", '⨑'),
    ("&bNot;", '⫭'),
    ("&backcong;", '≌'),
    ("&backepsilon;", '϶'),
    ("&backprime;", '‵'),
    ("&backsim;", '∽'),
    ("&backsimeq;", '⋍'),
    ("&barvee;", '⊽'),
    ("&barwed;", '⌅'),
    ("&barwedge;", '⌅'),
    ("&bbrk;", '⎵'),
    ("&bbrktbrk;", '⎶'),
    ("&bcong;", '≌'),
    ("&bcy;", 'б'),
    ("&bdquo;", '„'),
    ("&becaus;", '∵'),
    ("&because;", '∵'),
    ("&bemptyv;", '⦰'),
    ("&bepsi;", '϶'),
    ("&bernou;", 'ℬ'),
    ("&beta;", 'β'),
    ("&beth;", 'ℶ'),
    ("&between;", '≬'),
    ("&bfr;", '𝔟'),
    ("&bigcap;", '⋂'),
    ("&bigcirc;", '◯'),
    ("&bigcup;", '⋃'),
    ("&bigodot;", '⨀'),
    ("&bigoplus;", '⨁'),
    ("&bigotimes;", '⨂'),
    ("&bigsqcup;", '⨆'),
    ("&bigstar;", '★'),
    ("&bigtriangledown;", '▽'),
    ("&bigtriangleup;", '△'),
    ("&biguplus;", '⨄'),
    ("&bigvee;", '⋁'),
    ("&bigwedge;", '⋀'),
    ("&bkarow;", '⤍'),
    ("&blacklozenge;", '⧫'),
    ("&blacksquare;", '▪'),
    ("&blacktriangle;", '▴'),
    ("&blacktriangledown;", '▾'),
    ("&blacktriangleleft;", '◂'),
    ("&blacktriangleright;", '▸'),
    ("&blank;", '␣'),
    ("&blk12;", '▒'),
    ("&blk14;", '░'),
    ("&blk34;", '▓'),
    ("&block;", '█'),
    ("&bnot;", '⌐'),
    ("&bopf;", '𝕓'),
    ("&bot;", '⊥'),
    ("&bottom;", '⊥'),
    ("&bowtie;", '⋈'),
    ("&boxDL;", '╗'),
    ("&boxDR;", '╔'),
    ("&boxDl;", '╖'),
    ("&boxDr;", '╓'),
    ("&boxH;", '═'),
    ("&boxHD;", '╦'),
    ("&boxHU;", '╩'),
    ("&boxHd;", '╤'),
    ("&boxHu;", '╧'),
    ("&boxUL;", '╝'),
    ("&boxUR;", '╚'),
    ("&boxUl;", '╜'),
    ("&boxUr;", '╙'),
    ("&boxV;", '║'),
    ("&boxVH;", '╬'),
    ("&boxVL;", '╣'),
    ("&boxVR;", '╠'),
    ("&boxVh;", '╫'),
    ("&boxVl;", '╢'),
    ("&boxVr;", '╟'),
    ("&boxbox;", '⧉'),
    ("&boxdL;", '╕'),
    ("&boxdR;", '╒'),
    ("&boxdl;", '┐'),
    ("&boxdr;", '┌'),
    ("&boxh;", '─'),
    ("&boxhD;", '╥'),
    ("&boxhU;", '╨'),
    ("&boxhd;", '┬'),
    ("&boxhu;", '┴'),
    ("&boxminus;", '⊟'),
    ("&boxplus;", '⊞'),
    ("&boxtimes;", '⊠'),
    ("&boxuL;", '╛'),
    ("&boxuR;", '╘'),
    ("&boxul;", '┘'),
    ("&boxur;", '└'),
    ("&boxv;", '│'),
    ("&boxvH;", '╪'),
    ("&boxvL;", '╡'),
    ("&boxvR;", '╞'),
    ("&boxvh;", '┼'),
    ("&boxvl;", '┤'),
    ("&boxvr;", '├'),
    ("&bprime;", '‵'),
    ("&breve;", '˘'),
    ("&brvbar;", '¦'),
    ("&bscr;", '𝒷'),
    ("&bsemi;", '⁏'),
    ("&bsim;", '∽'),
    ("&bsime;", '⋍'),
    ("&bsol;", '\\'),
    ("&bsolb;", '⧅'),
    ("&bull;", '•'),
    ("&bullet;", '•'),
    ("&bump;", '≎'),
    ("&bumpE;", '⪮'),
    ("&bumpe;", '≏'),
    ("&bumpeq;", '≏'),
    ("&cacute;", 'ć'),
    ("&cap;", '∩'),
    ("&capand;", '⩄'),
    ("&capbrcup;", '⩉'),
    ("&capcap;", '⩋'),
    ("&capcup;", '⩇'),
    ("&capdot;", '⩀'),
    ("&caret;", '⁁'),
    ("&caron;", 'ˇ'),
    ("&ccaps;", '⩍'),
    ("&ccaron;", 'č'),
    ("&ccedil;", 'ç'),
    ("&ccirc;", 'ĉ'),
    ("&ccups;", '⩌'),
    ("&ccupssm;", '⩐'),
    ("&cdot;", 'ċ'),
    ("&cedil;", '¸'),
    ("&cemptyv;", '⦲'),
    ("&cent;", '¢'),
    ("&centerdot;", '·'),
    ("&cfr;", '𝔠'),
    ("&chcy;", 'ч'),
    ("&check;", '✓'),
    ("&checkmark;", '✓'),
    ("&chi;", 'χ'),
    ("&cir;", '○'),
    ("&cirE;", '⧃'),
    ("&circ;", 'ˆ'),
    ("&circeq;", '≗'),
    ("&circlearrowleft;", '↺'),
    ("&circlearrowright;", '↻'),
    ("&circledR;", '®'),
    ("&circledS;", 'Ⓢ'),
    ("&circledast;", '⊛'),
    ("&circledcirc;", '⊚'),
    ("&circleddash;", '⊝'),
    ("&cire;", '≗'),
    ("&cirfnint;", '⨐'),
    ("&cirmid;", '⫯'),
    ("&cirscir;", '⧂'),
    ("&clubs;", '♣'),
    ("&clubsuit;", '♣'),
    ("&colon;", ':'),
    ("&colone;", '≔'),
    ("&coloneq;", '≔'),
    ("&comma;", ','),
    ("&commat;", '@'),
    ("&comp;", '∁'),
    ("&compfn;", '∘'),
    ("&complement;", '∁'),
    ("&complexes;", 'ℂ'),
    ("&cong;", '≅'),
    ("&congdot;", '⩭'),
    ("&conint;", '∮'),
    ("&copf;", '𝕔'),
    ("&coprod;", '∐'),
    ("&copy;", '©'),
    ("&copysr;", '℗'),
    ("&crarr;", '↵'),
    ("&cross;", '✗'),
    ("&cscr;", '𝒸'),
    ("&csub;", '⫏'),
    ("&csube;", '⫑'),
    ("&csup;", '⫐'),
    ("&csupe;", '⫒'),
    ("&ctdot;", '⋯'),
    ("&cudarrl;", '⤸'),
    ("&cudarrr;", '⤵'),
    ("&cuepr;", '⋞'),
    ("&cuesc;", '⋟'),
    ("&cularr;", '↶'),
    ("&cularrp;", '⤽'),
    ("&cup;", '∪'),
    ("&cupbrcap;", '⩈'),
    ("&cupcap;", '⩆'),
    ("&cupcup;", '⩊'),
    ("&cupdot;", '⊍'),
    ("&cupor;", '⩅'),
    ("&curarr;", '↷'),
    ("&curarrm;", '⤼'),
    ("&curlyeqprec;", '⋞'),
    ("&curlyeqsucc;", '⋟'),
    ("&curlyvee;", '⋎'),
    ("&curlywedge;", '⋏'),
    ("&curren;", '¤'),
    ("&curvearrowleft;", '↶'),
    ("&curvearrowright;", '↷'),
    ("&cuvee;", '⋎'),
    ("&cuwed;", '⋏'),
    ("&cwconint;", '∲'),
    ("&cwint;", '∱'),
    ("&cylcty;", '⌭'),
    ("&dArr;", '⇓'),
    ("&dHar;", '⥥'),
    ("&dagger;", '†'),
    ("&daleth;", 'ℸ'),
    ("&darr;", '↓'),
    ("&dash;", '‐'),
    ("&dashv;", '⊣'),
    ("&dbkarow;", '⤏'),
    ("&dblac;", '˝'),
    ("&dcaron;", 'ď'),
    ("&dcy;", 'д'),
    ("&dd;", 'ⅆ'),
    ("&ddagger;", '‡'),
    ("&ddarr;", '⇊'),
    ("&ddotseq;", '⩷'),
    ("&deg;", '°'),
    ("&delta;", 'δ'),
    ("&demptyv;", '⦱'),
    ("&dfisht;", '⥿'),
    ("&dfr;", '𝔡'),
    ("&dharl;", '⇃'),
    ("&dharr;", '⇂'),
    ("&diam;", '⋄'),
    ("&diamond;", '⋄'),
    ("&diamondsuit;", '♦'),
    ("&diams;", '♦'),
    ("&die;", '¨'),
    ("&digamma;", 'ϝ'),
    ("&disin;", '⋲'),
    ("&div;", '÷'),
    ("&divide;", '÷'),
    ("&divideontimes;", '⋇'),
    ("&divonx;", '⋇'),
    ("&djcy;", 'ђ'),
    ("&dlcorn;", '⌞'),
    ("&dlcrop;", '⌍'),
    ("&dollar;", '$'),
    ("&dopf;", '𝕕'),
    ("&dot;", '˙'),
    ("&doteq;", '≐'),
    ("&doteqdot;", '≑'),
    ("&dotminus;", '∸'),
    ("&dotplus;", '∔'),
    ("&dotsquare;", '⊡'),
    ("&doublebarwedge;", '⌆'),
    ("&downarrow;", '↓'),
    ("&downdownarrows;", '⇊'),
    ("&downharpoonleft;", '⇃'),
    ("&downharpoonright;", '⇂'),
    ("&drbkarow;", '⤐'),
    ("&drcorn;", '⌟'),
    ("&drcrop;", '⌌'),
    ("&dscr;", '𝒹'),
    ("&dscy;", 'ѕ'),
    ("&dsol;", '⧶'),
    ("&dstrok;", 'đ'),
    ("&dtdot;", '⋱'),
    ("&dtri;", '▿'),
    ("&dtrif;", '▾'),
    ("&duarr;", '⇵'),
    ("&duhar;", '⥯'),
    ("&dwangle;", '⦦'),
    ("&dzcy;", 'џ'),
    ("&dzigrarr;", '⟿'),
    ("&eDDot;", '⩷'),
    ("&eDot;", '≑'),
    ("&eacute;", 'é'),
    ("&easter;", '⩮'),
    ("&ecaron;", 'ě'),
    ("&ecir;", '≖'),
    ("&ecirc;", 'ê'),
    ("&ecolon;", '≕'),
    ("&ecy;", 'э'),
    ("&edot;", 'ė'),
    ("&ee;", 'ⅇ'),
    ("&efDot;", '≒'),
    ("&efr;", '𝔢'),
    ("&eg;", '⪚'),
    ("&egrave;", 'è'),
    ("&egs;", '⪖'),
    ("&egsdot;", '⪘'),
    ("&el;", '⪙'),
    ("&elinters;", '⏧'),
    ("&ell;", 'ℓ'),
    ("&els;", '⪕'),
    ("&elsdot;", '⪗'),
    ("&emacr;", 'ē'),
    ("&empty;", '∅'),
    ("&emptyset;", '∅'),
    ("&emptyv;", '∅'),
    ("&emsp13;", '\u{8196}'),
    ("&emsp14;", '\u{8197}'),
    ("&emsp;", '\u{8195}'),
    ("&eng;", 'ŋ'),
    ("&ensp;", '\u{8194}'),
    ("&eogon;", 'ę'),
    ("&eopf;", '𝕖'),
    ("&epar;", '⋕'),
    ("&eparsl;", '⧣'),
    ("&eplus;", '⩱'),
    ("&epsi;", 'ϵ'),
    ("&epsilon;", 'ε'),
    ("&epsiv;", 'ε'),
    ("&eqcirc;", '≖'),
    ("&eqcolon;", '≕'),
    ("&eqsim;", '≂'),
    ("&eqslantgtr;", '⪖'),
    ("&eqslantless;", '⪕'),
    ("&equals;", '='),
    ("&equest;", '≟'),
    ("&equiv;", '≡'),
    ("&equivDD;", '⩸'),
    ("&eqvparsl;", '⧥'),
    ("&erDot;", '≓'),
    ("&erarr;", '⥱'),
    ("&escr;", 'ℯ'),
    ("&esdot;", '≐'),
    ("&esim;", '≂'),
    ("&eta;", 'η'),
    ("&eth;", 'ð'),
    ("&euml;", 'ë'),
    ("&euro;", '€'),
    ("&excl;", '!'),
    ("&exist;", '∃'),
    ("&expectation;", 'ℰ'),
    ("&exponentiale;", 'ⅇ'),
    ("&fallingdotseq;", '≒'),
    ("&fcy;", 'ф'),
    ("&female;", '♀'),
    ("&ffilig;", 'ﬃ'),
    ("&fflig;", 'ﬀ'),
    ("&ffllig;", 'ﬄ'),
    ("&ffr;", '𝔣'),
    ("&filig;", 'ﬁ'),
    ("&flat;", '♭'),
    ("&fllig;", 'ﬂ'),
    ("&fltns;", '▱'),
    ("&fnof;", 'ƒ'),
    ("&fopf;", '𝕗'),
    ("&forall;", '∀'),
    ("&fork;", '⋔'),
    ("&forkv;", '⫙'),
    ("&fpartint;", '⨍'),
    ("&frac12;", '½'),
    ("&frac13;", '⅓'),
    ("&frac14;", '¼'),
    ("&frac15;", '⅕'),
    ("&frac16;", '⅙'),
    ("&frac18;", '⅛'),
    ("&frac23;", '⅔'),
    ("&frac25;", '⅖'),
    ("&frac34;", '¾'),
    ("&frac35;", '⅗'),
    ("&frac38;", '⅜'),
    ("&frac45;", '⅘'),
    ("&frac56;", '⅚'),
    ("&frac58;", '⅝'),
    ("&frac78;", '⅞'),
    ("&frasl;", '⁄'),
    ("&frown;", '⌢'),
    ("&fscr;", '𝒻'),
    ("&gE;", '≧'),
    ("&gEl;", '⪌'),
    ("&gacute;", 'ǵ'),
    ("&gamma;", 'γ'),
    ("&gammad;", 'ϝ'),
    ("&gap;", '⪆'),
    ("&gbreve;", 'ğ'),
    ("&gcirc;", 'ĝ'),
    ("&gcy;", 'г'),
    ("&gdot;", 'ġ'),
    ("&ge;", '≥'),
    ("&gel;", '⋛'),
    ("&geq;", '≥'),
    ("&geqq;", '≧'),
    ("&geqslant;", '⩾'),
    ("&ges;", '⩾'),
    ("&gescc;", '⪩'),
    ("&gesdot;", '⪀'),
    ("&gesdoto;", '⪂'),
    ("&gesdotol;", '⪄'),
    ("&gesles;", '⪔'),
    ("&gfr;", '𝔤'),
    ("&gg;", '≫'),
    ("&ggg;", '⋙'),
    ("&gimel;", 'ℷ'),
    ("&gjcy;", 'ѓ'),
    ("&gl;", '≷'),
    ("&glE;", '⪒'),
    ("&gla;", '⪥'),
    ("&glj;", '⪤'),
    ("&gnE;", '≩'),
    ("&gnap;", '⪊'),
    ("&gnapprox;", '⪊'),
    ("&gne;", '⪈'),
    ("&gneq;", '⪈'),
    ("&gneqq;", '≩'),
    ("&gnsim;", '⋧'),
    ("&gopf;", '𝕘'),
    ("&grave;", '`'),
    ("&gscr;", 'ℊ'),
    ("&gsim;", '≳'),
    ("&gsime;", '⪎'),
    ("&gsiml;", '⪐'),
    ("&gt;", '>'),
    ("&gtcc;", '⪧'),
    ("&gtcir;", '⩺'),
    ("&gtdot;", '⋗'),
    ("&gtlPar;", '⦕'),
    ("&gtquest;", '⩼'),
    ("&gtrapprox;", '⪆'),
    ("&gtrarr;", '⥸'),
    ("&gtrdot;", '⋗'),
    ("&gtreqless;", '⋛'),
    ("&gtreqqless;", '⪌'),
    ("&gtrless;", '≷'),
    ("&gtrsim;", '≳'),
    ("&hArr;", '⇔'),
    ("&hairsp;", '\u{8202}'),
    ("&half;", '½'),
    ("&hamilt;", 'ℋ'),
    ("&hardcy;", 'ъ'),
    ("&harr;", '↔'),
    ("&harrcir;", '⥈'),
    ("&harrw;", '↭'),
    ("&hbar;", 'ℏ'),
    ("&hcirc;", 'ĥ'),
    ("&hearts;", '♥'),
    ("&heartsuit;", '♥'),
    ("&hellip;", '…'),
    ("&hercon;", '⊹'),
    ("&hfr;", '𝔥'),
    ("&hksearow;", '⤥'),
    ("&hkswarow;", '⤦'),
    ("&hoarr;", '⇿'),
    ("&homtht;", '∻'),
    ("&hookleftarrow;", '↩'),
    ("&hookrightarrow;", '↪'),
    ("&hopf;", '𝕙'),
    ("&horbar;", '―'),
    ("&hscr;", '𝒽'),
    ("&hslash;", 'ℏ'),
    ("&hstrok;", 'ħ'),
    ("&hybull;", '⁃'),
    ("&hyphen;", '‐'),
    ("&iacute;", 'í'),
    ("&ic;", '⁣'),
    ("&icirc;", 'î'),
    ("&icy;", 'и'),
    ("&iecy;", 'е'),
    ("&iexcl;", '¡'),
    ("&iff;", '⇔'),
    ("&ifr;", '𝔦'),
    ("&igrave;", 'ì'),
    ("&ii;", 'ⅈ'),
    ("&iiiint;", '⨌'),
    ("&iiint;", '∭'),
    ("&iinfin;", '⧜'),
    ("&iiota;", '℩'),
    ("&ijlig;", 'ĳ'),
    ("&imacr;", 'ī'),
    ("&image;", 'ℑ'),
    ("&imagline;", 'ℐ'),
    ("&imagpart;", 'ℑ'),
    ("&imath;", 'ı'),
    ("&imof;", '⊷'),
    ("&imped;", 'Ƶ'),
    ("&in;", '∈'),
    ("&incare;", '℅'),
    ("&infin;", '∞'),
    ("&infintie;", '⧝'),
    ("&inodot;", 'ı'),
    ("&int;", '∫'),
    ("&intcal;", '⊺'),
    ("&integers;", 'ℤ'),
    ("&intercal;", '⊺'),
    ("&intlarhk;", '⨗'),
    ("&intprod;", '⨼'),
    ("&iocy;", 'ё'),
    ("&iogon;", 'į'),
    ("&iopf;", '𝕚'),
    ("&iota;", 'ι'),
    ("&iprod;", '⨼'),
    ("&iquest;", '¿'),
    ("&iscr;", '𝒾'),
    ("&isin;", '∈'),
    ("&isinE;", '⋹'),
    ("&isindot;", '⋵'),
    ("&isins;", '⋴'),
    ("&isinsv;", '⋳'),
    ("&isinv;", '∈'),
    ("&it;", '⁢'),
    ("&itilde;", 'ĩ'),
    ("&iukcy;", 'і'),
    ("&iuml;", 'ï'),
    ("&jcirc;", 'ĵ'),
    ("&jcy;", 'й'),
    ("&jfr;", '𝔧'),
    ("&jmath;", 'ȷ'),
    ("&jopf;", '𝕛'),
    ("&jscr;", '𝒿'),
    ("&jsercy;", 'ј'),
    ("&jukcy;", 'є'),
    ("&kappa;", 'κ'),
    ("&kappav;", 'ϰ'),
    ("&kcedil;", 'ķ'),
    ("&kcy;", 'к'),
    ("&kfr;", '𝔨'),
    ("&kgreen;", 'ĸ'),
    ("&khcy;", 'х'),
    ("&kjcy;", 'ќ'),
    ("&kopf;", '𝕜'),
    ("&kscr;", '𝓀'),
    ("&lAarr;", '⇚'),
    ("&lArr;", '⇐'),
    ("&lAtail;", '⤛'),
    ("&lBarr;", '⤎'),
    ("&lE;", '≦'),
    ("&lEg;", '⪋'),
    ("&lHar;", '⥢'),
    ("&lacute;", 'ĺ'),
    ("&laemptyv;", '⦴'),
    ("&lagran;", 'ℒ'),
    ("&lambda;", 'λ'),
    ("&lang;", '⟨'),
    ("&langd;", '⦑'),
    ("&langle;", '⟨'),
    ("&lap;", '⪅'),
    ("&laquo;", '«'),
    ("&larr;", '←'),
    ("&larrb;", '⇤'),
    ("&larrbfs;", '⤟'),
    ("&larrfs;", '⤝'),
    ("&larrhk;", '↩'),
    ("&larrlp;", '↫'),
    ("&larrpl;", '⤹'),
    ("&larrsim;", '⥳'),
    ("&larrtl;", '↢'),
    ("&lat;", '⪫'),
    ("&latail;", '⤙'),
    ("&late;", '⪭'),
    ("&lbarr;", '⤌'),
    ("&lbbrk;", '❲'),
    ("&lbrace;", '{'),
    ("&lbrack;", '['),
    ("&lbrke;", '⦋'),
    ("&lbrksld;", '⦏'),
    ("&lbrkslu;", '⦍'),
    ("&lcaron;", 'ľ'),
    ("&lcedil;", 'ļ'),
    ("&lceil;", '⌈'),
    ("&lcub;", '{'),
    ("&lcy;", 'л'),
    ("&ldca;", '⤶'),
    ("&ldquo;", '“'),
    ("&ldquor;", '„'),
    ("&ldrdhar;", '⥧'),
    ("&ldrushar;", '⥋'),
    ("&ldsh;", '↲'),
    ("&le;", '≤'),
    ("&leftarrow;", '←'),
    ("&leftarrowtail;", '↢'),
    ("&leftharpoondown;", '↽'),
    ("&leftharpoonup;", '↼'),
    ("&leftleftarrows;", '⇇'),
    ("&leftrightarrow;", '↔'),
    ("&leftrightarrows;", '⇆'),
    ("&leftrightharpoons;", '⇋'),
    ("&leftrightsquigarrow;", '↭'),
    ("&leftthreetimes;", '⋋'),
    ("&leg;", '⋚'),
    ("&leq;", '≤'),
    ("&leqq;", '≦'),
    ("&leqslant;", '⩽'),
    ("&les;", '⩽'),
    ("&lescc;", '⪨'),
    ("&lesdot;", '⩿'),
    ("&lesdoto;", '⪁'),
    ("&lesdotor;", '⪃'),
    ("&lesges;", '⪓'),
    ("&lessapprox;", '⪅'),
    ("&lessdot;", '⋖'),
    ("&lesseqgtr;", '⋚'),
    ("&lesseqqgtr;", '⪋'),
    ("&lessgtr;", '≶'),
    ("&lesssim;", '≲'),
    ("&lfisht;", '⥼'),
    ("&lfloor;", '⌊'),
    ("&lfr;", '𝔩'),
    ("&lg;", '≶'),
    ("&lgE;", '⪑'),
    ("&lhard;", '↽'),
    ("&lharu;", '↼'),
    ("&lharul;", '⥪'),
    ("&lhblk;", '▄'),
    ("&ljcy;", 'љ'),
    ("&ll;", '≪'),
    ("&llarr;", '⇇'),
    ("&llcorner;", '⌞'),
    ("&llhard;", '⥫'),
    ("&lltri;", '◺'),
    ("&lmidot;", 'ŀ'),
    ("&lmoust;", '⎰'),
    ("&lmoustache;", '⎰'),
    ("&lnE;", '≨'),
    ("&lnap;", '⪉'),
    ("&lnapprox;", '⪉'),
    ("&lne;", '⪇'),
    ("&lneq;", '⪇'),
    ("&lneqq;", '≨'),
    ("&lnsim;", '⋦'),
    ("&loang;", '⟬'),
    ("&loarr;", '⇽'),
    ("&lobrk;", '⟦'),
    ("&longleftarrow;", '⟵'),
    ("&longleftrightarrow;", '⟷'),
    ("&longmapsto;", '⟼'),
    ("&longrightarrow;", '⟶'),
    ("&looparrowleft;", '↫'),
    ("&looparrowright;", '↬'),
    ("&lopar;", '⦅'),
    ("&lopf;", '𝕝'),
    ("&loplus;", '⨭'),
    ("&lotimes;", '⨴'),
    ("&lowast;", '∗'),
    ("&lowbar;", '_'),
    ("&loz;", '◊'),
    ("&lozenge;", '◊'),
    ("&lozf;", '⧫'),
    ("&lpar;", '('),
    ("&lparlt;", '⦓'),
    ("&lrarr;", '⇆'),
    ("&lrcorner;", '⌟'),
    ("&lrhar;", '⇋'),
    ("&lrhard;", '⥭'),
    ("&lrm;", '‎'),
    ("&lrtri;", '⊿'),
    ("&lsaquo;", '‹'),
    ("&lscr;", '𝓁'),
    ("&lsh;", '↰'),
    ("&lsim;", '≲'),
    ("&lsime;", '⪍'),
    ("&lsimg;", '⪏'),
    ("&lsqb;", '['),
    ("&lsquo;", '‘'),
    ("&lsquor;", '‚'),
    ("&lstrok;", 'ł'),
    ("&lt;", '<'),
    ("&ltcc;", '⪦'),
    ("&ltcir;", '⩹'),
    ("&ltdot;", '⋖'),
    ("&lthree;", '⋋'),
    ("&ltimes;", '⋉'),
    ("&ltlarr;", '⥶'),
    ("&ltquest;", '⩻'),
    ("&ltrPar;", '⦖'),
    ("&ltri;", '◃'),
    ("&ltrie;", '⊴'),
    ("&ltrif;", '◂'),
    ("&lurdshar;", '⥊'),
    ("&luruhar;", '⥦'),
    ("&mDDot;", '∺'),
    ("&macr;", '¯'),
    ("&male;", '♂'),
    ("&malt;", '✠'),
    ("&maltese;", '✠'),
    ("&map;", '↦'),
    ("&mapsto;", '↦'),
    ("&mapstodown;", '↧'),
    ("&mapstoleft;", '↤'),
    ("&mapstoup;", '↥'),
    ("&marker;", '▮'),
    ("&mcomma;", '⨩'),
    ("&mcy;", 'м'),
    ("&mdash;", '—'),
    ("&measuredangle;", '∡'),
    ("&mfr;", '𝔪'),
    ("&mho;", '℧'),
    ("&micro;", 'µ'),
    ("&mid;", '∣'),
    ("&midast;", '*'),
    ("&midcir;", '⫰'),
    ("&middot;", '·'),
    ("&minus;", '−'),
    ("&minusb;", '⊟'),
    ("&minusd;", '∸'),
    ("&minusdu;", '⨪'),
    ("&mlcp;", '⫛'),
    ("&mldr;", '…'),
    ("&mnplus;", '∓'),
    ("&models;", '⊧'),
    ("&mopf;", '𝕞'),
    ("&mp;", '∓'),
    ("&mscr;", '𝓂'),
    ("&mstpos;", '∾'),
    ("&mu;", 'μ'),
    ("&multimap;", '⊸'),
    ("&mumap;", '⊸'),
    ("&nLeftarrow;", '⇍'),
    ("&nLeftrightarrow;", '⇎'),
    ("&nRightarrow;", '⇏'),
    ("&nVDash;", '⊯'),
    ("&nVdash;", '⊮'),
    ("&nabla;", '∇'),
    ("&nacute;", 'ń'),
    ("&nap;", '≉'),
    ("&napos;", 'ŉ'),
    ("&napprox;", '≉'),
    ("&natur;", '♮'),
    ("&natural;", '♮'),
    ("&naturals;", 'ℕ'),
    ("&nbsp;", '\u{160}'),
    ("&ncap;", '⩃'),
    ("&ncaron;", 'ň'),
    ("&ncedil;", 'ņ'),
    ("&ncong;", '≇'),
    ("&ncup;", '⩂'),
    ("&ncy;", 'н'),
    ("&ndash;", '–'),
    ("&ne;", '≠'),
    ("&neArr;", '⇗'),
    ("&nearhk;", '⤤'),
    ("&nearr;", '↗'),
    ("&nearrow;", '↗'),
    ("&nequiv;", '≢'),
    ("&nesear;", '⤨'),
    ("&nexist;", '∄'),
    ("&nexists;", '∄'),
    ("&nfr;", '𝔫'),
    ("&nge;", '≱'),
    ("&ngeq;", '≱'),
    ("&ngsim;", '≵'),
    ("&ngt;", '≯'),
    ("&ngtr;", '≯'),
    ("&nhArr;", '⇎'),
    ("&nharr;", '↮'),
    ("&nhpar;", '⫲'),
    ("&ni;", '∋'),
    ("&nis;", '⋼'),
    ("&nisd;", '⋺'),
    ("&niv;", '∋'),
    ("&njcy;", 'њ'),
    ("&nlArr;", '⇍'),
    ("&nlarr;", '↚'),
    ("&nldr;", '‥'),
    ("&nle;", '≰'),
    ("&nleftarrow;", '↚'),
    ("&nleftrightarrow;", '↮'),
    ("&nleq;", '≰'),
    ("&nless;", '≮'),
    ("&nlsim;", '≴'),
    ("&nlt;", '≮'),
    ("&nltri;", '⋪'),
    ("&nltrie;", '⋬'),
    ("&nmid;", '∤'),
    ("&nopf;", '𝕟'),
    ("&not;", '¬'),
    ("&notin;", '∉'),
    ("&notinva;", '∉'),
    ("&notinvb;", '⋷'),
    ("&notinvc;", '⋶'),
    ("&notni;", '∌'),
    ("&notniva;", '∌'),
    ("&notnivb;", '⋾'),
    ("&notnivc;", '⋽'),
    ("&npar;", '∦'),
    ("&nparallel;", '∦'),
    ("&npolint;", '⨔'),
    ("&npr;", '⊀'),
    ("&nprcue;", '⋠'),
    ("&nprec;", '⊀'),
    ("&nrArr;", '⇏'),
    ("&nrarr;", '↛'),
    ("&nrightarrow;", '↛'),
    ("&nrtri;", '⋫'),
    ("&nrtrie;", '⋭'),
    ("&nsc;", '⊁'),
    ("&nsccue;", '⋡'),
    ("&nscr;", '𝓃'),
    ("&nshortmid;", '∤'),
    ("&nshortparallel;", '∦'),
    ("&nsim;", '≁'),
    ("&nsime;", '≄'),
    ("&nsimeq;", '≄'),
    ("&nsmid;", '∤'),
    ("&nspar;", '∦'),
    ("&nsqsube;", '⋢'),
    ("&nsqsupe;", '⋣'),
    ("&nsub;", '⊄'),
    ("&nsube;", '⊈'),
    ("&nsubseteq;", '⊈'),
    ("&nsucc;", '⊁'),
    ("&nsup;", '⊅'),
    ("&nsupe;", '⊉'),
    ("&nsupseteq;", '⊉'),
    ("&ntgl;", '≹'),
    ("&ntilde;", 'ñ'),
    ("&ntlg;", '≸'),
    ("&ntriangleleft;", '⋪'),
    ("&ntrianglelefteq;", '⋬'),
    ("&ntriangleright;", '⋫'),
    ("&ntrianglerighteq;", '⋭'),
    ("&nu;", 'ν'),
    ("&num;", '#'),
    ("&numero;", '№'),
    ("&numsp;", '\u{8199}'),
    ("&nvDash;", '⊭'),
    ("&nvHarr;", '⤄'),
    ("&nvdash;", '⊬'),
    ("&nvinfin;", '⧞'),
    ("&nvlArr;", '⤂'),
    ("&nvrArr;", '⤃'),
    ("&nwArr;", '⇖'),
    ("&nwarhk;", '⤣'),
    ("&nwarr;", '↖'),
    ("&nwarrow;", '↖'),
    ("&nwnear;", '⤧'),
    ("&oS;", 'Ⓢ'),
    ("&oacute;", 'ó'),
    ("&oast;", '⊛'),
    ("&ocir;", '⊚'),
    ("&ocirc;", 'ô'),
    ("&ocy;", 'о'),
    ("&odash;", '⊝'),
    ("&odblac;", 'ő'),
    ("&odiv;", '⨸'),
    ("&odot;", '⊙'),
    ("&odsold;", '⦼'),
    ("&oelig;", 'œ'),
    ("&ofcir;", '⦿'),
    ("&ofr;", '𝔬'),
    ("&ogon;", '˛'),
    ("&ograve;", 'ò'),
    ("&ogt;", '⧁'),
    ("&ohbar;", '⦵'),
    ("&ohm;", 'Ω'),
    ("&oint;", '∮'),
    ("&olarr;", '↺'),
    ("&olcir;", '⦾'),
    ("&olcross;", '⦻'),
    ("&oline;", '‾'),
    ("&olt;", '⧀'),
    ("&omacr;", 'ō'),
    ("&omega;", 'ω'),
    ("&omicron;", 'ο'),
    ("&omid;", '⦶'),
    ("&ominus;", '⊖'),
    ("&oopf;", '𝕠'),
    ("&opar;", '⦷'),
    ("&operp;", '⦹'),
    ("&oplus;", '⊕'),
    ("&or;", '∨'),
    ("&orarr;", '↻'),
    ("&ord;", '⩝'),
    ("&order;", 'ℴ'),
    ("&orderof;", 'ℴ'),
    ("&ordf;", 'ª'),
    ("&ordm;", 'º'),
    ("&origof;", '⊶'),
    ("&oror;", '⩖'),
    ("&orslope;", '⩗'),
    ("&orv;", '⩛'),
    ("&oscr;", 'ℴ'),
    ("&oslash;", 'ø'),
    ("&osol;", '⊘'),
    ("&otilde;", 'õ'),
    ("&otimes;", '⊗'),
    ("&otimesas;", '⨶'),
    ("&ouml;", 'ö'),
    ("&ovbar;", '⌽'),
    ("&par;", '∥'),
    ("&para;", '¶'),
    ("&parallel;", '∥'),
    ("&parsim;", '⫳'),
    ("&parsl;", '⫽'),
    ("&part;", '∂'),
    ("&pcy;", 'п'),
    ("&percnt;", '%'),
    ("&period;", '.'),
    ("&permil;", '‰'),
    ("&perp;", '⊥'),
    ("&pertenk;", '‱'),
    ("&pfr;", '𝔭'),
    ("&phi;", 'φ'),
    ("&phiv;", 'φ'),
    ("&phmmat;", 'ℳ'),
    ("&phone;", '☎'),
    ("&pi;", 'π'),
    ("&pitchfork;", '⋔'),
    ("&piv;", 'ϖ'),
    ("&planck;", 'ℏ'),
    ("&planckh;", 'ℎ'),
    ("&plankv;", 'ℏ'),
    ("&plus;", '+'),
    ("&plusacir;", '⨣'),
    ("&plusb;", '⊞'),
    ("&pluscir;", '⨢'),
    ("&plusdo;", '∔'),
    ("&plusdu;", '⨥'),
    ("&pluse;", '⩲'),
    ("&plusmn;", '±'),
    ("&plussim;", '⨦'),
    ("&plustwo;", '⨧'),
    ("&pm;", '±'),
    ("&pointint;", '⨕'),
    ("&popf;", '𝕡'),
    ("&pound;", '£'),
    ("&pr;", '≺'),
    ("&prE;", '⪳'),
    ("&prap;", '⪷'),
    ("&prcue;", '≼'),
    ("&pre;", '⪯'),
    ("&prec;", '≺'),
    ("&precapprox;", '⪷'),
    ("&preccurlyeq;", '≼'),
    ("&preceq;", '⪯'),
    ("&precnapprox;", '⪹'),
    ("&precneqq;", '⪵'),
    ("&precnsim;", '⋨'),
    ("&precsim;", '≾'),
    ("&prime;", '′'),
    ("&primes;", 'ℙ'),
    ("&prnE;", '⪵'),
    ("&prnap;", '⪹'),
    ("&prnsim;", '⋨'),
    ("&prod;", '∏'),
    ("&profalar;", '⌮'),
    ("&profline;", '⌒'),
    ("&profsurf;", '⌓'),
    ("&prop;", '∝'),
    ("&propto;", '∝'),
    ("&prsim;", '≾'),
    ("&prurel;", '⊰'),
    ("&pscr;", '𝓅'),
    ("&psi;", 'ψ'),
    ("&puncsp;", '\u{8200}'),
    ("&qfr;", '𝔮'),
    ("&qint;", '⨌'),
    ("&qopf;", '𝕢'),
    ("&qprime;", '⁗'),
    ("&qscr;", '𝓆'),
    ("&quaternions;", 'ℍ'),
    ("&quatint;", '⨖'),
    ("&quest;", '?'),
    ("&questeq;", '≟'),
    ("&quot;", '"'),
    ("&rAarr;", '⇛'),
    ("&rArr;", '⇒'),
    ("&rAtail;", '⤜'),
    ("&rBarr;", '⤏'),
    ("&rHar;", '⥤'),
    ("&race;", '⧚'),
    ("&racute;", 'ŕ'),
    ("&radic;", '√'),
    ("&raemptyv;", '⦳'),
    ("&rang;", '⟩'),
    ("&rangd;", '⦒'),
    ("&range;", '⦥'),
    ("&rangle;", '⟩'),
    ("&raquo;", '»'),
    ("&rarr;", '→'),
    ("&rarrap;", '⥵'),
    ("&rarrb;", '⇥'),
    ("&rarrbfs;", '⤠'),
    ("&rarrc;", '⤳'),
    ("&rarrfs;", '⤞'),
    ("&rarrhk;", '↪'),
    ("&rarrlp;", '↬'),
    ("&rarrpl;", '⥅'),
    ("&rarrsim;", '⥴'),
    ("&rarrtl;", '↣'),
    ("&rarrw;", '↝'),
    ("&ratail;", '⤚'),
    ("&ratio;", '∶'),
    ("&rationals;", 'ℚ'),
    ("&rbarr;", '⤍'),
    ("&rbbrk;", '❳'),
    ("&rbrace;", '}'),
    ("&rbrack;", ']'),
    ("&rbrke;", '⦌'),
    ("&rbrksld;", '⦎'),
    ("&rbrkslu;", '⦐'),
    ("&rcaron;", 'ř'),
    ("&rcedil;", 'ŗ'),
    ("&rceil;", '⌉'),
    ("&rcub;", '}'),
    ("&rcy;", 'р'),
    ("&rdca;", '⤷'),
    ("&rdldhar;", '⥩'),
    ("&rdquo;", '”'),
    ("&rdquor;", '”'),
    ("&rdsh;", '↳'),
    ("&real;", 'ℜ'),
    ("&realine;", 'ℛ'),
    ("&realpart;", 'ℜ'),
    ("&reals;", 'ℝ'),
    ("&rect;", '▭'),
    ("&reg;", '®'),
    ("&rfisht;", '⥽'),
    ("&rfloor;", '⌋'),
    ("&rfr;", '𝔯'),
    ("&rhard;", '⇁'),
    ("&rharu;", '⇀'),
    ("&rharul;", '⥬'),
    ("&rho;", 'ρ'),
    ("&rhov;", 'ϱ'),
    ("&rightarrow;", '→'),
    ("&rightarrowtail;", '↣'),
    ("&rightharpoondown;", '⇁'),
    ("&rightharpoonup;", '⇀'),
    ("&rightleftarrows;", '⇄'),
    ("&rightleftharpoons;", '⇌'),
    ("&rightrightarrows;", '⇉'),
    ("&rightsquigarrow;", '↝'),
    ("&rightthreetimes;", '⋌'),
    ("&ring;", '˚'),
    ("&risingdotseq;", '≓'),
    ("&rlarr;", '⇄'),
    ("&rlhar;", '⇌'),
    ("&rlm;", '‏'),
    ("&rmoust;", '⎱'),
    ("&rmoustache;", '⎱'),
    ("&rnmid;", '⫮'),
    ("&roang;", '⟭'),
    ("&roarr;", '⇾'),
    ("&robrk;", '⟧'),
    ("&ropar;", '⦆'),
    ("&ropf;", '𝕣'),
    ("&roplus;", '⨮'),
    ("&rotimes;", '⨵'),
    ("&rpar;", ')'),
    ("&rpargt;", '⦔'),
    ("&rppolint;", '⨒'),
    ("&rrarr;", '⇉'),
    ("&rsaquo;", '›'),
    ("&rscr;", '𝓇'),
    ("&rsh;", '↱'),
    ("&rsqb;", ']'),
    ("&rsquo;", '’'),
    ("&rsquor;", '’'),
    ("&rthree;", '⋌'),
    ("&rtimes;", '⋊'),
    ("&rtri;", '▹'),
    ("&rtrie;", '⊵'),
    ("&rtrif;", '▸'),
    ("&rtriltri;", '⧎'),
    ("&ruluhar;", '⥨'),
    ("&rx;", '℞'),
    ("&sacute;", 'ś'),
    ("&sbquo;", '‚'),
    ("&sc;", '≻'),
    ("&scE;", '⪴'),
    ("&scap;", '⪸'),
    ("&scaron;", 'š'),
    ("&sccue;", '≽'),
    ("&sce;", '⪰'),
    ("&scedil;", 'ş'),
    ("&scirc;", 'ŝ'),
    ("&scnE;", '⪶'),
    ("&scnap;", '⪺'),
    ("&scnsim;", '⋩'),
    ("&scpolint;", '⨓'),
    ("&scsim;", '≿'),
    ("&scy;", 'с'),
    ("&sdot;", '⋅'),
    ("&sdotb;", '⊡'),
    ("&sdote;", '⩦'),
    ("&seArr;", '⇘'),
    ("&searhk;", '⤥'),
    ("&searr;", '↘'),
    ("&searrow;", '↘'),
    ("&sect;", '§'),
    ("&semi;", ';'),
    ("&seswar;", '⤩'),
    ("&setminus;", '∖'),
    ("&setmn;", '∖'),
    ("&sext;", '✶'),
    ("&sfr;", '𝔰'),
    ("&sfrown;", '⌢'),
    ("&sharp;", '♯'),
    ("&shchcy;", 'щ'),
    ("&shcy;", 'ш'),
    ("&shortmid;", '∣'),
    ("&shortparallel;", '∥'),
    ("&shy;", '­'),
    ("&sigma;", 'σ'),
    ("&sigmaf;", 'ς'),
    ("&sigmav;", 'ς'),
    ("&sim;", '∼'),
    ("&simdot;", '⩪'),
    ("&sime;", '≃'),
    ("&simeq;", '≃'),
    ("&simg;", '⪞'),
    ("&simgE;", '⪠'),
    ("&siml;", '⪝'),
    ("&simlE;", '⪟'),
    ("&simne;", '≆'),
    ("&simplus;", '⨤'),
    ("&simrarr;", '⥲'),
    ("&slarr;", '←'),
    ("&smallsetminus;", '∖'),
    ("&smashp;", '⨳'),
    ("&smeparsl;", '⧤'),
    ("&smid;", '∣'),
    ("&smile;", '⌣'),
    ("&smt;", '⪪'),
    ("&smte;", '⪬'),
    ("&softcy;", 'ь'),
    ("&sol;", '/'),
    ("&solb;", '⧄'),
    ("&solbar;", '⌿'),
    ("&sopf;", '𝕤'),
    ("&spades;", '♠'),
    ("&spadesuit;", '♠'),
    ("&spar;", '∥'),
    ("&sqcap;", '⊓'),
    ("&sqcup;", '⊔'),
    ("&sqsub;", '⊏'),
    ("&sqsube;", '⊑'),
    ("&sqsubset;", '⊏'),
    ("&sqsubseteq;", '⊑'),
    ("&sqsup;", '⊐'),
    ("&sqsupe;", '⊒'),
    ("&sqsupset;", '⊐'),
    ("&sqsupseteq;", '⊒'),
    ("&squ;", '□'),
    ("&square;", '□'),
    ("&squarf;", '▪'),
    ("&squf;", '▪'),
    ("&srarr;", '→'),
    ("&sscr;", '𝓈'),
    ("&ssetmn;", '∖'),
    ("&ssmile;", '⌣'),
    ("&sstarf;", '⋆'),
    ("&star;", '☆'),
    ("&starf;", '★'),
    ("&straightepsilon;", 'ϵ'),
    ("&straightphi;", 'ϕ'),
    ("&strns;", '¯'),
    ("&sub;", '⊂'),
    ("&subE;", '⫅'),
    ("&subdot;", '⪽'),
    ("&sube;", '⊆'),
    ("&subedot;", '⫃'),
    ("&submult;", '⫁'),
    ("&subnE;", '⫋'),
    ("&subne;", '⊊'),
    ("&subplus;", '⪿'),
    ("&subrarr;", '⥹'),
    ("&subset;", '⊂'),
    ("&subseteq;", '⊆'),
    ("&subseteqq;", '⫅'),
    ("&subsetneq;", '⊊'),
    ("&subsetneqq;", '⫋'),
    ("&subsim;", '⫇'),
    ("&subsub;", '⫕'),
    ("&subsup;", '⫓'),
    ("&succ;", '≻'),
    ("&succapprox;", '⪸'),
    ("&succcurlyeq;", '≽'),
    ("&succeq;", '⪰'),
    ("&succnapprox;", '⪺'),
    ("&succneqq;", '⪶'),
    ("&succnsim;", '⋩'),
    ("&succsim;", '≿'),
    ("&sum;", '∑'),
    ("&sung;", '♪'),
    ("&sup1;", '¹'),
    ("&sup2;", '²'),
    ("&sup3;", '³'),
    ("&sup;", '⊃'),
    ("&supE;", '⫆'),
    ("&supdot;", '⪾'),
    ("&supdsub;", '⫘'),
    ("&supe;", '⊇'),
    ("&supedot;", '⫄'),
    ("&suphsub;", '⫗'),
    ("&suplarr;", '⥻'),
    ("&supmult;", '⫂'),
    ("&supnE;", '⫌'),
    ("&supne;", '⊋'),
    ("&supplus;", '⫀'),
    ("&supset;", '⊃'),
    ("&supseteq;", '⊇'),
    ("&supseteqq;", '⫆'),
    ("&supsetneq;", '⊋'),
    ("&supsetneqq;", '⫌'),
    ("&supsim;", '⫈'),
    ("&supsub;", '⫔'),
    ("&supsup;", '⫖'),
    ("&swArr;", '⇙'),
    ("&swarhk;", '⤦'),
    ("&swarr;", '↙'),
    ("&swarrow;", '↙'),
    ("&swnwar;", '⤪'),
    ("&szlig;", 'ß'),
    ("&target;", '⌖'),
    ("&tau;", 'τ'),
    ("&tbrk;", '⎴'),
    ("&tcaron;", 'ť'),
    ("&tcedil;", 'ţ'),
    ("&tcy;", 'т'),
    ("&tdot;", '⃛'),
    ("&telrec;", '⌕'),
    ("&tfr;", '𝔱'),
    ("&there4;", '∴'),
    ("&therefore;", '∴'),
    ("&theta;", 'θ'),
    ("&thetasym;", 'ϑ'),
    ("&thetav;", 'ϑ'),
    ("&thickapprox;", '≈'),
    ("&thicksim;", '∼'),
    ("&thinsp;", '\u{8201}'),
    ("&thkap;", '≈'),
    ("&thksim;", '∼'),
    ("&thorn;", 'þ'),
    ("&tilde;", '˜'),
    ("&times;", '×'),
    ("&timesb;", '⊠'),
    ("&timesbar;", '⨱'),
    ("&timesd;", '⨰'),
    ("&tint;", '∭'),
    ("&toea;", '⤨'),
    ("&top;", '⊤'),
    ("&topbot;", '⌶'),
    ("&topcir;", '⫱'),
    ("&topf;", '𝕥'),
    ("&topfork;", '⫚'),
    ("&tosa;", '⤩'),
    ("&tprime;", '‴'),
    ("&trade;", '™'),
    ("&triangle;", '▵'),
    ("&triangledown;", '▿'),
    ("&triangleleft;", '◃'),
    ("&trianglelefteq;", '⊴'),
    ("&triangleq;", '≜'),
    ("&triangleright;", '▹'),
    ("&trianglerighteq;", '⊵'),
    ("&tridot;", '◬'),
    ("&trie;", '≜'),
    ("&triminus;", '⨺'),
    ("&triplus;", '⨹'),
    ("&trisb;", '⧍'),
    ("&tritime;", '⨻'),
    ("&trpezium;", '⏢'),
    ("&tscr;", '𝓉'),
    ("&tscy;", 'ц'),
    ("&tshcy;", 'ћ'),
    ("&tstrok;", 'ŧ'),
    ("&twixt;", '≬'),
    ("&twoheadleftarrow;", '↞'),
    ("&twoheadrightarrow;", '↠'),
    ("&uArr;", '⇑'),
    ("&uHar;", '⥣'),
    ("&uacute;", 'ú'),
    ("&uarr;", '↑'),
    ("&ubrcy;", 'ў'),
    ("&ubreve;", 'ŭ'),
    ("&ucirc;", 'û'),
    ("&ucy;", 'у'),
    ("&udarr;", '⇅'),
    ("&udblac;", 'ű'),
    ("&udhar;", '⥮'),
    ("&ufisht;", '⥾'),
    ("&ufr;", '𝔲'),
    ("&ugrave;", 'ù'),
    ("&uharl;", '↿'),
    ("&uharr;", '↾'),
    ("&uhblk;", '▀'),
    ("&ulcorn;", '⌜'),
    ("&ulcorner;", '⌜'),
    ("&ulcrop;", '⌏'),
    ("&ultri;", '◸'),
    ("&umacr;", 'ū'),
    ("&uml;", '¨'),
    ("&uogon;", 'ų'),
    ("&uopf;", '𝕦'),
    ("&uparrow;", '↑'),
    ("&updownarrow;", '↕'),
    ("&upharpoonleft;", '↿'),
    ("&upharpoonright;", '↾'),
    ("&uplus;", '⊎'),
    ("&upsi;", 'υ'),
    ("&upsih;", 'ϒ'),
    ("&upsilon;", 'υ'),
    ("&upuparrows;", '⇈'),
    ("&urcorn;", '⌝'),
    ("&urcorner;", '⌝'),
    ("&urcrop;", '⌎'),
    ("&uring;", 'ů'),
    ("&urtri;", '◹'),
    ("&uscr;", '𝓊'),
    ("&utdot;", '⋰'),
    ("&utilde;", 'ũ'),
    ("&utri;", '▵'),
    ("&utrif;", '▴'),
    ("&uuarr;", '⇈'),
    ("&uuml;", 'ü'),
    ("&uwangle;", '⦧'),
    ("&vArr;", '⇕'),
    ("&vBar;", '⫨'),
    ("&vBarv;", '⫩'),
    ("&vDash;", '⊨'),
    ("&vangrt;", '⦜'),
    ("&varepsilon;", 'ε'),
    ("&varkappa;", 'ϰ'),
    ("&varnothing;", '∅'),
    ("&varphi;", 'φ'),
    ("&varpi;", 'ϖ'),
    ("&varpropto;", '∝'),
    ("&varr;", '↕'),
    ("&varrho;", 'ϱ'),
    ("&varsigma;", 'ς'),
    ("&vartheta;", 'ϑ'),
    ("&vartriangleleft;", '⊲'),
    ("&vartriangleright;", '⊳'),
    ("&vcy;", 'в'),
    ("&vdash;", '⊢'),
    ("&vee;", '∨'),
    ("&veebar;", '⊻'),
    ("&veeeq;", '≚'),
    ("&vellip;", '⋮'),
    ("&verbar;", '|'),
    ("&vert;", '|'),
    ("&vfr;", '𝔳'),
    ("&vltri;", '⊲'),
    ("&vopf;", '𝕧'),
    ("&vprop;", '∝'),
    ("&vrtri;", '⊳'),
    ("&vscr;", '𝓋'),
    ("&vzigzag;", '⦚'),
    ("&wcirc;", 'ŵ'),
    ("&wedbar;", '⩟'),
    ("&wedge;", '∧'),
    ("&wedgeq;", '≙'),
    ("&weierp;", '℘'),
    ("&wfr;", '𝔴'),
    ("&wopf;", '𝕨'),
    ("&wp;", '℘'),
    ("&wr;", '≀'),
    ("&wreath;", '≀'),
    ("&wscr;", '𝓌'),
    ("&xcap;", '⋂'),
    ("&xcirc;", '◯'),
    ("&xcup;", '⋃'),
    ("&xdtri;", '▽'),
    ("&xfr;", '𝔵'),
    ("&xhArr;", '⟺'),
    ("&xharr;", '⟷'),
    ("&xi;", 'ξ'),
    ("&xlArr;", '⟸'),
    ("&xlarr;", '⟵'),
    ("&xmap;", '⟼'),
    ("&xnis;", '⋻'),
    ("&xodot;", '⨀'),
    ("&xopf;", '𝕩'),
    ("&xoplus;", '⨁'),
    ("&xotime;", '⨂'),
    ("&xrArr;", '⟹'),
    ("&xrarr;", '⟶'),
    ("&xscr;", '𝓍'),
    ("&xsqcup;", '⨆'),
    ("&xuplus;", '⨄'),
    ("&xutri;", '△'),
    ("&xvee;", '⋁'),
    ("&xwedge;", '⋀'),
    ("&yacute;", 'ý'),
    ("&yacy;", 'я'),
    ("&ycirc;", 'ŷ'),
    ("&ycy;", 'ы'),
    ("&yen;", '¥'),
    ("&yfr;", '𝔶'),
    ("&yicy;", 'ї'),
    ("&yopf;", '𝕪'),
    ("&yscr;", '𝓎'),
    ("&yucy;", 'ю'),
    ("&yuml;", 'ÿ'),
    ("&zacute;", 'ź'),
    ("&zcaron;", 'ž'),
    ("&zcy;", 'з'),
    ("&zdot;", 'ż'),
    ("&zeetrf;", 'ℨ'),
    ("&zeta;", 'ζ'),
    ("&zfr;", '𝔷'),
    ("&zhcy;", 'ж'),
    ("&zigrarr;", '⇝'),
    ("&zopf;", '𝕫'),
    ("&zscr;", '𝓏'),
    ("&zwj;", '‍'),
    ("&zwnj;", '\u{8204}'),
];


pub trait HTMLEntity {
    type EscapeOutput;
    type UnEscapeOutput;

    fn escape_html(self) -> Self::EscapeOutput;
    fn unescape_html(self) -> Self::UnEscapeOutput;
}


impl HTMLEntity for &Vec<char> {
    type EscapeOutput = Vec<char>;
    type UnEscapeOutput = Vec<char>;

    fn escape_html(self) -> Self::EscapeOutput {
        let mut idx = 0usize;
        
        let input = self;
        let input_len = input.len();
        let mut output: Vec<char> = Vec::with_capacity(input_len);
        
        loop {
            if idx == input_len {
                break;
            }
            
            let c = input[idx];
            match c {
                '&' => {
                    // FIXME: 是否需要忽略已经 转义 过的序列？
                    output.extend_from_slice(&['&', 'A', 'M', 'P', ';']);
                },
                '>' => {
                    output.extend_from_slice(&['&', 'G', 'T', ';']);
                },
                '<' => {
                    output.extend_from_slice(&['&', 'L', 'T', ';']);
                },
                '"' => {
                    output.extend_from_slice(&['&', 'Q', 'U', 'O', 'T', ';']);
                },
                '\\' => {
                    output.extend_from_slice(&['&', 'b', 's', 'o', 'l', ';']);
                },
                _ => {
                    output.push(c);
                }
            }

            idx += 1;
        }

        output
    }
    
    fn unescape_html(self) -> Self::UnEscapeOutput {
        let mut idx = 0usize;
        
        let input = self;
        let input_len = input.len();
        let mut output: Vec<char> = Vec::with_capacity(input_len);
        
        loop {
            if idx == input_len {
                break;
            }

            let c = input[idx];
            match c {
                '&' => {
                    let mut idx2 = idx + 1;
                    match input.get(idx2) {
                        Some('#') => {
                            idx2 += 1;
                            
                            let mut escaped_char: Option<char> = None;

                            match input.get(idx2) {
                                Some(c3) => {
                                    let is_hex_number = if c3 == &'x' {
                                        idx2 += 1;
                                        true
                                    } else {
                                        false
                                    };

                                    if is_hex_number || c3.is_ascii_digit() {
                                        // read number
                                        loop {
                                            match input.get(idx2) {
                                                Some(c4) => {
                                                    if c4 == &';' {
                                                        if is_hex_number {
                                                            let s = &input[idx+3..idx2].iter().collect::<String>();
                                                            match u32::from_str_radix(&s, 16).ok().and_then(std::char::from_u32) {
                                                                Some(ch) => {
                                                                    escaped_char = Some(ch);
                                                                    idx = idx2;
                                                                },
                                                                None => {
                                                                    // FIXME: DEBUG ?
                                                                },
                                                            }
                                                        } else {
                                                            let s = &input[idx+2..idx2].iter().collect::<String>();
                                                            match u32::from_str_radix(&s, 10).ok().and_then(std::char::from_u32) {
                                                                Some(ch) => {
                                                                    escaped_char = Some(ch);
                                                                    idx = idx2;
                                                                },
                                                                None => {
                                                                    // FIXME: DEBUG ?
                                                                },
                                                            }
                                                        }

                                                        break;
                                                    }

                                                    if idx2 - idx >= 10 {
                                                        break;
                                                    }

                                                    if is_hex_number {
                                                        if !c4.is_ascii_hexdigit() {
                                                            break;
                                                        }
                                                    } else {
                                                        if !c4.is_ascii_digit() {
                                                            break;
                                                        }
                                                    }

                                                    idx2 += 1;
                                                },
                                                None => {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                },
                                None => { },
                            }

                            match escaped_char {
                                Some(ch) => {
                                    output.push(ch);
                                },
                                None => {
                                    output.push(c);
                                },
                            }
                        },
                        Some(c3) => {
                            let mut escaped_char: Option<char> = None;

                            // UnicodeXID::is_xid_continue(*c3)
                            if c3.is_ascii_alphabetic() {
                                // read name
                                loop {
                                    idx2 += 1;
                                    
                                    if idx2 - idx >= MAX_NAME_LENGTH + 2 {
                                        break;
                                    }

                                    match input.get(idx2) {
                                        Some(c4) => {
                                            if c4 == &';' {
                                                let s = &input[idx..idx2+1].iter().collect::<String>();
                                                let ident = s.as_str();
                                                match NAMED_ENTITIES.binary_search_by_key(&ident, |&(name, _)| name) {
                                                    Ok(pos) => {
                                                        let item = NAMED_ENTITIES[pos];
                                                        escaped_char = Some(item.1);
                                                        idx = idx2;
                                                    },
                                                    Err(_) => {

                                                    },
                                                }

                                                break;
                                            }

                                            if !c4.is_ascii_alphabetic() {
                                                break;
                                            }

                                        },
                                        None => {
                                            break;
                                        },
                                    }

                                }
                            }
                            
                            match escaped_char {
                                Some(ch) => {
                                    output.push(ch);
                                },
                                None => {
                                    output.push(c);
                                },
                            }
                        },
                        None => {
                            output.push(c);
                        },
                    }
                },
                _ => {
                    output.push(c);
                },
            }

            idx += 1;
        }

        output
    }
}

impl HTMLEntity for &[u8] {
    type EscapeOutput = Vec<u8>;
    type UnEscapeOutput = Vec<u8>;

    fn escape_html(self) -> Self::EscapeOutput {
        let mut idx = 0usize;
        
        let input = self;
        let input_utf8_len = input.len();
        let mut output = Vec::with_capacity(input_utf8_len);
        
        loop {
            if idx == input_utf8_len {
                break;
            }
            
            let byte = input[idx];
            match byte {
                b'&' => {
                    // FIXME: 是否需要忽略已经 转义 过的序列？
                    output.extend_from_slice("&AMP;".as_bytes());
                },
                b'>' => {
                    output.extend_from_slice("&GT;".as_bytes());
                },
                b'<' => {
                    output.extend_from_slice("&LT;".as_bytes());
                },
                b'"' => {
                    output.extend_from_slice("&QUOT;".as_bytes());
                },
                b'\\' => {
                    output.extend_from_slice("&bsol;".as_bytes());
                },
                _ => {
                    output.push(byte);
                }
            }

            idx += 1;
        }

        output
    }

    fn unescape_html(self) -> Self::UnEscapeOutput {
        let mut idx = 0usize;
        
        let input = self;
        let input_len = input.len();
        let mut output: Vec<u8> = Vec::with_capacity(input_len);
        

        loop {
            if idx == input_len {
                break;
            }

            let c = input[idx];
            match c {
                b'&' => {
                    let mut idx2 = idx + 1;
                    match input.get(idx2) {
                        Some(b'#') => {
                            idx2 += 1;
                            
                            let mut escaped_char: Option<char> = None;

                            match input.get(idx2) {
                                Some(c3) => {
                                    let is_hex_number = if c3 == &b'x' {
                                        idx2 += 1;
                                        true
                                    } else {
                                        false
                                    };

                                    if is_hex_number || c3.is_ascii_digit() {
                                        // read number
                                        loop {
                                            match input.get(idx2) {
                                                Some(c4) => {
                                                    if c4 == &b';' {
                                                        if is_hex_number {
                                                            let s = unsafe { std::str::from_utf8_unchecked(&input[idx+3..idx2]) };
                                                            match u32::from_str_radix(&s, 16).ok().and_then(std::char::from_u32) {
                                                                Some(ch) => {
                                                                    escaped_char = Some(ch);
                                                                    idx = idx2;
                                                                },
                                                                None => {
                                                                    // FIXME: DEBUG ?
                                                                },
                                                            }
                                                        } else {
                                                            let s = unsafe { std::str::from_utf8_unchecked(&input[idx+2..idx2]) };
                                                            match u32::from_str_radix(&s, 10).ok().and_then(std::char::from_u32) {
                                                                Some(ch) => {
                                                                    escaped_char = Some(ch);
                                                                    idx = idx2;
                                                                },
                                                                None => {
                                                                    // FIXME: DEBUG ?
                                                                },
                                                            }
                                                        }

                                                        break;
                                                    }

                                                    if idx2 - idx >= 10 {
                                                        break;
                                                    }

                                                    if is_hex_number {
                                                        if !c4.is_ascii_hexdigit() {
                                                            break;
                                                        }
                                                    } else {
                                                        if !c4.is_ascii_digit() {
                                                            break;
                                                        }
                                                    }

                                                    idx2 += 1;
                                                },
                                                None => {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                },
                                None => { },
                            }

                            match escaped_char {
                                Some(ch) => {
                                    let mut buffer = [0u8; 4];
                                    output.extend_from_slice(ch.encode_utf8(&mut buffer).as_bytes());
                                },
                                None => {
                                    output.push(c);
                                },
                            }
                        },
                        Some(c3) => {
                            let mut escaped_char: Option<char> = None;
                            
                            if c3.is_ascii_alphabetic() {
                                // read name
                                loop {
                                    idx2 += 1;
                                    
                                    if idx2 - idx >= MAX_NAME_LENGTH + 2 {
                                        break;
                                    }

                                    match input.get(idx2) {
                                        Some(c4) => {
                                            if c4 == &b';' {

                                                let ident = unsafe { std::str::from_utf8_unchecked(&input[idx..idx2+1]) };

                                                match NAMED_ENTITIES.binary_search_by_key(&ident, |&(name, _)| name) {
                                                    Ok(pos) => {
                                                        let item = NAMED_ENTITIES[pos];
                                                        escaped_char = Some(item.1);
                                                        idx = idx2;
                                                    },
                                                    Err(_) => {

                                                    },
                                                }

                                                break;
                                            }

                                            if !c4.is_ascii_alphabetic() {
                                                break;
                                            }

                                        },
                                        None => {
                                            break;
                                        },
                                    }

                                }
                            }
                            
                            match escaped_char {
                                Some(ch) => {
                                    let mut buffer = [0u8; 4];
                                    output.extend_from_slice(ch.encode_utf8(&mut buffer).as_bytes());
                                },
                                None => {
                                    output.push(c);
                                },
                            }
                        },
                        None => {
                            output.push(c);
                        },
                    }
                },
                _ => {
                    output.push(c);
                },
            }

            idx += 1;
        }

        output
    }
}

impl HTMLEntity for &str {
    type EscapeOutput = String;
    type UnEscapeOutput = String;

    fn escape_html(self) -> Self::EscapeOutput {
        unsafe { String::from_utf8_unchecked(self.as_bytes().escape_html()) }
    }

    fn unescape_html(self) -> Self::UnEscapeOutput {
        unsafe { String::from_utf8_unchecked(self.as_bytes().unescape_html()) }
    }
}

impl HTMLEntity for Vec<u8> {
    type EscapeOutput = Vec<u8>;
    type UnEscapeOutput = Vec<u8>;

    fn escape_html(self) -> Self::EscapeOutput {
        (&self).escape_html()
    }

    fn unescape_html(self) -> Self::UnEscapeOutput {
        (&self).unescape_html()
    }
}

impl HTMLEntity for &Vec<u8> {
    type EscapeOutput = Vec<u8>;
    type UnEscapeOutput = Vec<u8>;

    fn escape_html(self) -> Self::EscapeOutput {
        self.as_slice().escape_html()
    }

    fn unescape_html(self) -> Self::UnEscapeOutput {
        self.as_slice().unescape_html()
    }
}



#[test]
fn test_escape_html() {
    let s = "<App>who am i?</App>".chars().collect::<Vec<char>>();
    let escaped = s.escape_html().iter().collect::<String>();
    assert_eq!(escaped, "&LT;App&GT;who am i?&LT;/App&GT;");

    let s = "<App>who am i?</App>".as_bytes();
    let r = s.escape_html();
    let escaped = unsafe { std::str::from_utf8_unchecked(r.as_slice()) };
    assert_eq!(escaped, "&LT;App&GT;who am i?&LT;/App&GT;");

    let s = "<App>who am i?</App>";
    let escaped = s.escape_html();
    assert_eq!(escaped.as_str(), "&LT;App&GT;who am i?&LT;/App&GT;");
}

#[test]
fn test_unescape_html() {
    let s = "&LT;App&GT;who am i?&LT;/App&GT; &Xopf; &#x1D54F; &#120143; &#120143 &#x1D54F &Xopf &#Xopf".chars().collect::<Vec<char>>();
    let unescaped = s.unescape_html().iter().collect::<String>();
    assert_eq!(unescaped, "<App>who am i?</App> 𝕏 𝕏 𝕏 &#120143 &#x1D54F &Xopf &#Xopf");

    let s = "&LT;App&GT;who am i?&LT;/App&GT; &Xopf; &#x1D54F; &#120143; &#120143 &#x1D54F &Xopf &#Xopf".as_bytes();
    let r = s.unescape_html();
    let unescaped = unsafe { std::str::from_utf8_unchecked(r.as_slice()) };
    assert_eq!(unescaped, "<App>who am i?</App> 𝕏 𝕏 𝕏 &#120143 &#x1D54F &Xopf &#Xopf");

    let s = "&LT;App&GT;who am i?&LT;/App&GT; &Xopf; &#x1D54F; &#120143; &#120143 &#x1D54F &Xopf &#Xopf";
    let unescaped = s.unescape_html();
    assert_eq!(unescaped.as_str(), "<App>who am i?</App> 𝕏 𝕏 𝕏 &#120143 &#x1D54F &Xopf &#Xopf");
}


#[bench]
fn bench_escape_html_with_str(b: &mut test::Bencher) {
    static BIG_STR: &'static str = include_str!("../../data/moonstone-short.txt");
    
    b.bytes = BIG_STR.len() as u64;

    b.iter(|| {
        let _ = BIG_STR.escape_html();
    });
}

#[bench]
fn bench_escape_html_with_bytes(b: &mut test::Bencher) {
    static BIG_STR: &[u8] = include_bytes!("../../data/moonstone-short.txt");
    
    b.bytes = BIG_STR.len() as u64;

    b.iter(|| {
        let _ = BIG_STR.escape_html();
    });
}

#[bench]
fn bench_escape_html_with_chars(b: &mut test::Bencher) {
    let BIG_STR: Vec<char> = include_str!("../../data/moonstone-short.txt").chars().collect::<Vec<char>>();
    
    b.bytes = BIG_STR.len() as u64;

    b.iter(|| {
        let _ = BIG_STR.escape_html();
    });
}

#[bench]
fn bench_unescape_html_with_chars(b: &mut test::Bencher) {
    let BIG_STR: Vec<char> = include_str!("../../data/moonstone-short.txt").chars().collect::<Vec<char>>();
    let escaped = BIG_STR.escape_html();

    let amt = escaped.iter().collect::<String>().len();

    b.bytes = amt as u64;
    b.iter(|| {
        let _ = escaped.unescape_html();
    });
}

#[bench]
fn bench_unescape_html_with_bytes(b: &mut test::Bencher) {
    static BIG_STR: &[u8] = include_bytes!("../../data/moonstone-short.txt");
    let escaped = BIG_STR.escape_html();
    let escaped_slice = escaped.as_slice();

    b.bytes = escaped_slice.len() as u64;
    b.iter(|| {
        let _ = escaped_slice.unescape_html();
    });
}

#[bench]
fn bench_unescape_html_with_str(b: &mut test::Bencher) {
    let BIG_STR: &str = include_str!("../../data/moonstone-short.txt");
    let escaped = BIG_STR.escape_html();
    let escaped_str = escaped.as_str();

    b.bytes = escaped_str.len() as u64;
    b.iter(|| {
        let _ = escaped_str.unescape_html();
    });
}
