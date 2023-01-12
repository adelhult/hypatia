import styled from "styled-components";
import { AnimatePresence, motion } from "framer-motion";
import CodeMirror from "@uiw/react-codemirror";

const Container = styled(motion.div)`
  height: 100vh;
  padding: 2rem;
  font-weight: 300;
  flex-grow:0;
  flex-shrink:1;
  width: 100%;
  overflow-y: auto;
  background: #e9e6e2;
  box-sizing: border-box;

  &>h1 {
    font-size: 1.5rem;
   }

  &>h2 {
    font-size: 1.2rem;
    line-height: 1;
    margin-bottom:0.3rem;
    margin-top: 2rem;
    opacity: 0.5;
   }

   &>h3 {
    font-size: 1rem;
    font-weight: normal;
   }

  & code {
    font-family: 'JetBrains Mono', monospace;
    background-color: #00000010;
    padding: 0.1rem;
    border-radius: 0.3rem;
  }

  & ul {
    padding-left: 1rem;
    box-sizing: border-box;
  }

  & li {
    margin-bottom: 0.5rem;
  }

  @media (max-width: 800px) {
    height: auto;
    max-width: 100%;
  }
`;

const ExampleBlock = styled.div`
  margin-top: 0.3rem;
`;

type HelpProps = {
  show: boolean;
};

function Example({ value }: { value: string }) {
  return <ExampleBlock>
    <CodeMirror
      readOnly
      value={value}
      basicSetup={{
        highlightActiveLineGutter: false,
        lineNumbers: false,
        highlightActiveLine: false,
      }}
    >
    </CodeMirror>
  </ExampleBlock>;
}

export default function Help({ show }: HelpProps) {
  return (
    <AnimatePresence>
      {show
        ? (
          <Container
          >
            <h1>How to use</h1>
            <p>Welcome! Hypatia is a calculator language that's well suited for dealing with large numbers and unit conversions.</p>

            <p>It's easy to get started, just type out any mathematical expression and it will get evaluated. Here are a few examples:</p>
            <Example
              value={`10 + 20
2 km / 3
sin(20 degrees) * 3`}
            />
            <h2>Convert units</h2>
            To display the result of an expression in another compatible unit, simply use the <code>in</code> keyword.
            <Example
              value={`2 km + 3 mile in meter`}
            />
            <h2>Variables</h2>
            Declare variables with the <code>=</code> operator. Use the <code>update</code> keyword if you later want to
            reassign a new value to the same name.
            <Example
              value={`width = 20 m
height = 30 m
update height = 35 m
              
area = width * height`}
            />

            <h2>Literals</h2>
            Binary, hexadecimal and scientific literals are supported
            <Example
              value={`0xa2a3
0b101010
3e-2`}
            />

            <h2>Block expressions</h2>
            A pair of curly braces are used to create a block and a new variable
            scope. Variables declared inside of the block exits only within that
            block. The last expression in the block is what the block itself
            evaluates to.
            <Example
              value={`area = {
  width = 20 m
  height = 30 m
  width * height
}`}
            />
            <h2>Conditional expressions</h2>
            <Example
              value={`if x < 100 m {
  // ...
} else {
  // ...
}
              
if x > 3 km {
  // ...
} else if x == 100 m {
  // ...
}`}
            />
            <h2>Functions</h2>
            See the example for how to use functions.
            <Example
              value={`f(x) = 2 * x + 5

fibbonaci(n) = if n <= 1 {
  1
} else {
  fibbonaci(n - 1) + fibbonaci(n - 2)
}`}
            />
            <h2>Custom units</h2>
            You can easily use custom units. Use the <code>unit</code> keyword followed by a name (and an optional shorthand name) to declare a unit.
            <Example
              value={`unit kronor kr
unit dollar usd = 10 kr
10 usd + 50 kr`}
            />
            <h2>Custom prefixes</h2>
            Declare a custom unit prefix using the <code>prefix</code> keyword followed by an equals sign and a numeric value.
            <Example
              value={`prefix super = 10000000
15 supergram`}
            />
            <h1>Language reference</h1>
            Work in progress. Will write this once more of the language is completed.
            <h2>Keywords and operators</h2>
            <h3>General keywords</h3>
            <ul>
              <li><code>if</code></li>
              <li><code>else</code></li>
              <li><code>update</code></li>
              <li><code>in</code></li>
              <li><code>unit</code></li>
              <li><code>prefix</code></li>
              <li><code>=</code> (assignment)</li>
            </ul>
            <h3>Arithmetic operators</h3>
            <ul>
              <li><code>+</code> (add)</li>
              <li><code>-</code> (subtract or negate)</li>
              <li><code>/</code> (divide)</li>
              <li><code>*</code> (multiply)</li>
            </ul>

            <h3>Relational operators</h3>
            <ul>
              <li><code>&lt;</code> (less)</li>
              <li><code>&lt;=</code> (less than or equal)</li>
              <li><code>&gt;</code> (greater than)</li>
              <li><code>&gt;=</code> (greater than or equal)</li>
              <li><code>==</code> (equals)</li>
              <li><code>!=</code> (not equal)</li>
            </ul>
            <h3>Logical operators</h3>
            <ul>
              <li><code>and</code> (logical and)</li>
              <li><code>or</code> (logical or)</li>
              <li><code>xor</code> (logical exlusive or)</li>
              <li><code>not</code> (logical not)</li>
            </ul>




            <h2>Built-in functions</h2>
            <h2>Built-in units</h2>
            <h2>Built-in prefixes</h2>
          </Container>
        )
        : ""}
    </AnimatePresence>
  );
}
