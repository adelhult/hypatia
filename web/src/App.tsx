import Menu from "./Menu";
import CodeMirror from '@uiw/react-codemirror';
import styled from "styled-components";
import init, { evaluate } from 'web_bindings';
import { useEffect, useState } from "react";
import Convert from "ansi-to-html";

const Workspace = styled.div`
  width: 100%;
  max-width: 800px;
  margin-left:auto;
  margin-right:auto;
  box-sizing: border-box;
`;

const Result = styled.div`
  padding:1rem;
  font-size:0.8rem;
  line-height: 1;
  box-sizing: border-box;
  background-color: rgba(0,0,0, 0.05);
  font-family: monospace;
`;

function App() {
  const [loaded, setLoaded] = useState(false);
  const [source, setSource] = useState("");
  const [result, setResult] = useState("");

  // Load the WASM file
  useEffect(() => {
    init().then(() => setLoaded(true));
  }, []);

  useEffect(() => {
    if (!loaded) return;
    const output = evaluate(source);
    console.log(output);
    let converter = new Convert({newline: true}); 
    setResult(converter.toHtml(output));
  }, [source])
  
  return <div className="App">
      <Menu />
      {loaded && <Workspace>
        <CodeMirror
          onChange={setSource}
          value={source}
        />
        <pre><Result dangerouslySetInnerHTML={{__html: result}} /></pre>
      </Workspace>}
    </div>
}

export default App
