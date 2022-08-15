import Menu from "./Menu";
import CodeMirror from '@uiw/react-codemirror';
import styled from "styled-components";
import init, { greet } from 'web_bindings';
import { useEffect, useState } from "react";

const Workspace = styled.div`
  width: 100%;
  max-width: 800px;
  margin-left:auto;
  margin-right:auto;
  background:rgba(0,0,0,0.05);
  box-sizing: border-box;
`;


function App() {
  const [loaded, setLoaded] = useState(false);
  useEffect(() => {
    init().then(() => setLoaded(true));
  }, []);
  
  return loaded && <div className="App">
      <Menu />
      <Workspace>
        <h1 onClick={event =>greet("Eli")}>Hello</h1>
        <CodeMirror
          value="console.log('hello world!');"
        />
      </Workspace>
    </div>
}

export default App
