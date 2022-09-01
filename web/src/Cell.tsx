import React from "react";
import { createTheme } from '@uiw/codemirror-themes';
import {keymap} from "@codemirror/view"
import CodeMirror from '@uiw/react-codemirror';
import { tags as t } from '@lezer/highlight';
import styled from "styled-components";
import Convert from "ansi-to-html"; 

const Result = styled.div`
  padding:1rem;
  font-size:0.8rem;
  line-height: 1;
  box-sizing: border-box;
  background-color: rgba(0,0,0, 0.05);
  font-family: 'Roboto Mono', monospace;
  overflow-y: auto;
  margin-bottom: 3rem;
  margin-top: 0;
`;

const Wrapper = styled.div`
    box-shadow: 0 2px 2px rgba(0,0,0, 0.3);
`;


const hotkeys = (addCellAction: () => void) => keymap.of([{
      key: "Alt-Enter",
      run() { addCellAction(); return true }
    }]);

const theme = createTheme({
    theme: 'light',
    settings: {
      background: '#ffffff',
      foreground: '#1f1f1f',
      caret: '#AEAFAD',
      selection: '#D6D6D6',
      selectionMatch: '#D6D6D6',
      gutterBackground: '#FFFFFF',
      gutterForeground: '#4D4D4C',
      gutterBorder: '#ddd',
      lineHighlight: '#EFEFEF',
    },
    styles: [
      { tag: t.comment, color: '#787b80' },
      { tag: t.definition(t.typeName), color: '#194a7b' },
      { tag: t.typeName, color: '#194a7b' },
      { tag: t.tagName, color: '#008a02' },
      { tag: t.variableName, color: '#1a00db' },
    ],
});


interface CellProps {
    code: string,
    output: string,
    index: number,
    addCellAction: () => void,
    onChange: (cell_index: number, code: string) => void,
}

const Cell = React.memo((props: CellProps) => {
    const converter = new Convert();

    return <Wrapper>
        <CodeMirror
            onChange={code => props.onChange(props.index, code)}
            value={props.code}
            theme={theme}
            autoFocus
            extensions={[hotkeys(props.addCellAction)]}
            basicSetup={{
                lineNumbers: props.code.match("\n") != null,
            }}
        />
        {
            <pre><Result dangerouslySetInnerHTML={{__html: converter.toHtml(props.output)}} /></pre>
        }
    </Wrapper>
});

export default Cell;