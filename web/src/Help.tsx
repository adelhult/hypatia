import styled from "styled-components";
import { AnimatePresence, motion } from "framer-motion";
import CodeMirror from "@uiw/react-codemirror";

const Container = styled(motion.div)`
  height: 100vh;
  padding: 2rem;
  flex-grow: 0;
  font-weight: 300;
  max-width: 37rem;
  overflow-y: auto;
  background: #e9e6e2;
  box-sizing: border-box;
  &>h1 {
    font-size: 1.5rem;
   }

  &>h2 {
    font-size: 1.2rem;
    line-height: 1;
    margin-bottom:0.2rem;
    margin-top: 2rem;
   }
`;

type HelpProps = {
  show: boolean;
};

function Example({ value }: { value: string }) {
  return (
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
  );
}

export default function Help({ show }: HelpProps) {
  return (
    <AnimatePresence>
      {show
        ? (
          <Container
            transition={{ easings: ["easeIn", "easeOut"] }}
            initial={{ width: 0 }}
            animate={{ width: "100%" }}
            exit={{ width: 0 }}
          >
            <h1>How to use</h1>
            <h2>It's a calculator!</h2>
            Evaluate mathematical expressions
            <Example
              value={`10 + 20
2 km / 3
sin(20 degrees) * 3`}
            />
            <h2>Convert units</h2>
            The `-&gt;` keyword can be used to convert between compatible units
            <Example
              value={`2 km + 3 mile -> meter`}
            />
            <h2>Literals</h2>
            Binary, hexadecimal and scientific literals are supported
            <Example
              value={`0xa2a3
0b101010
3e-2`}
            />
            <h2>Variables</h2>
            Declare variables with the `=` operator. Use the `update` keyword to
            reassign a new value to the same name.
            <Example
              value={`width = 20 m
height = 30 m
update height = 35 m
              
area = width * height`}
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
              value={`if true {
// ...
} else {
  // ...
}
              
if true {
  // ...
} else if true {
  // ...
}`}
            />
            <h2>Functions</h2>
            <Example
              value={`f(x) = 2 * x + 5

fibbonaci(n) = if n <= 1 {
  1
} else {
  fibbonaci(n - 1) + fibbonaci(n - 2)
}`}
            />
            <h2>Custom units</h2>
            <Example
              value={`unit kronor kr
unit dollar usd = 10 kr
10 usd + 50 kr`}
            />
            <h2>Custom prefixes</h2>
            <Example
              value={`prefix super = 10000000
15 supergram`}
            />
            <h1>Language reference</h1>
            <h2>Keywords and operators</h2>
            <h2>Built-in functions</h2>
            <h2>Built-in units</h2>
            <h2>Built-in prefixes</h2>
          </Container>
        )
        : ""}
    </AnimatePresence>
  );
}
