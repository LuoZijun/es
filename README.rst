ECMAScript
==================================

.. contents::


Work in progress (WIP).


项目状态
----------

*   ✅ 词法分析器 (Lexer)
*   🦋 语法分析器 (Parser)
*   🔜 编译器 (Compiler)
*   🔜 解释器 (Interpreter)

**Lexer**:

*JSX*:

*   ❌ JSXText

**Parser**:

*   ✅ EmptyStatement
*   ✅ DebuggerStatement
*   ❌ Expression

    *   ✅ Identifier
    *   ✅ LiteralNull
    *   ✅ LiteralBoolean
    *   ✅ LiteralString
    *   ✅ LiteralNumeric
    *   ✅ LiteralRegularExpression
    *   ✅ LiteralTemplateExpression
    *   ❌ LiteralArray
    *   ❌ LiteralObject
    *   ✅ ThisExpression
    *   ✅ MemberExpression ( include `SuperMemberExpression` )
    *   ✅ NewTargetExpression
    *   ✅ NewExpression
    *   ✅ PrefixExpression
    *   ✅ InfixExpression
    *   ✅ PostfixExpression
    *   ✅ AssignmentExpression
    *   ✅ ConditionalExpression
    *   ✅ YieldExpression
    *   ✅ CallExpression ( include `SuperCallExpression` )
    *   ✅ TaggedTemplate
    *   ✅ SpreadExpression
    *   ✅ CommaExpression
    *   ✅ ParenthesizedExpression ( Grouping )
    *   ❌ ObjectBindingPattern ( Destructuring )
    *   ❌ ArrayBindingPattern ( Destructuring )
    *   ✅ ClassExpression
    *   ✅ FunctionExpression
    *   ✅ GeneratorExpression
    *   ✅ AsyncFunctionExpression
    *   ✅ AsyncGeneratorExpression
    *   ✅ ArrowFunctionExpression
    *   ✅ AsyncArrowFunctionExpression

*   ❌ VariableStatement
*   ✅ BlockStatement
*   ❌ IfStatement
*   ❌ DoWhileStatement
*   ❌ WhileStatement
*   ❌ ForStatement
*   ❌ ForInStatement
*   ❌ ForOfStatement
*   ❌ ForAwaitOfStatement
*   ❌ ContinueStatement
*   ❌ BreakStatement
*   ❌ ReturnStatement
*   ❌ WithStatement
*   ❌ SwitchStatement
*   ❌ LabelledStatement
*   ❌ ThrowStatement
*   ❌ TryStatement
*   ✅ FunctionDeclaration
*   ✅ ClassDeclaration

*JSX*:

*   ❌ JSXFragment
*   ❌ JSXElement ( JSXSelfClosingElement, JSXNormalElement, )

*Scripts and Modules*:

*   ❌ ImportDeclaration
*   ❌ ExportDeclaration


原则
----------

1. 完全尊重 ECMAScript 语言规范，规范之外的内容均不会考虑(JSX 将是唯一的一个例外)。
2. 语法分析器和解释器只支持严格模式，这意味着一些在非严格模式下的语法都将会直接抛出错误。
3. 分号自动补全问题，这个还没有定，个人倾向于必须写。


愿景
----------

1. 语法分析器完整支持 ECMAScript 已发布语法规范(Release)，处于提议阶段的语法特性或其它组织定义的语法都不会考虑（JSX例外）。
2. 编译器支持编译为 字节码(ByteCode) 和 低版本的源代码(ES2011 - ES2018)。
3. 实现一个简单的解释器。


友好的错误提示
---------------

.. code::

    ~/P/es> cargo run --example esc -- < in.js

    SyntaxError: Unexpected Character `/`
     --> src/main.js:1:1
      |
    1 | #/usr/bin/env python3
      |  ^


用例
---------

这是设想当中的使用方法，目前处于不可用状态。


.. code:: bash
    
    esc src/main.js
    esc src/index.html
    esc --to es2011 src/main.js
    esc --to es2011 --bundle src/main.js
    
    esi src/main.js


FAQ
--------

会考虑支持 ECMAScript 语言的一些超集或子集(如: TypeScript/Flow)吗？

    除了 `JSX` 其它的都不会考虑。

会考虑实现目前尚处于提议阶段的语法特性和内建API设计吗？

    不会，如果最终这些提议能够顺利进入发布阶段，那个时候肯定会去实现改提议的特性。

一些在现有生态代码里面非常常见的写法（如 Babel 支持它），但它不是 ECMAScript 规范的一部分，这样的语法特性会考虑兼容吗？

    不会。


License
---------

MIT license (LICENSE or http://opensource.org/licenses/MIT)
