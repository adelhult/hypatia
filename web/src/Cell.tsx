import React from "react";
import { createTheme } from '@uiw/codemirror-themes';
import {keymap} from "@codemirror/view"
import CodeMirror from '@uiw/react-codemirror';
import { tags as t } from '@lezer/highlight';
import styled from "styled-components";
import { MdClose } from "react-icons/md";
import Convert from "ansi-to-html"; 
import {motion} from "framer-motion";

const Result = styled.div`
    padding:0.5rem;
    box-sizing: border-box;
    background-color: #ededed;
    font-family: 'Roboto Mono', monospace;
    margin-top: 0;
`;

const ResultData = styled.div`
    font-size:1rem;
    line-height: 1;
    overflow-y: hidden;
    overflow-x: auto;
    
`;

const AnswerText = styled.span`
  opacity: 0.7;
  font-size: 0.8rem;  
`;

const Wrapper = styled(motion.div)`
    position: relative;
    border: solid;
    border-width: 1px;
    border-color: #d0d0d7;
    margin-bottom: 1rem;
    box-shadow: 0 2px 2px rgba(0,0,0, 0.1);
`;

const Remove = styled.button`
    z-index: 1000;
    position: absolute;
    right: 3px;
    top: 6px;
    font-size: 1rem;
    background: none;
    border: none;
    opacity: 0.5;
    cursor: pointer;
    transition: opacity 0.2s;

    &:hover {
        opacity: 1;
    }
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
    noAnimation?: boolean,
    addCellAction: () => void,
    onRemove: (cell_index: number) => void,
    onChange: (cell_index: number, code: string) => void,
}

const Cell = React.memo((props: CellProps) => {
    const converter = new Convert();

    return <Wrapper initial={!props.noAnimation && {y: -20, opacity: 0}} animate={{y: 0, opacity: 1}}>    
        <Remove title="Remove cell" onClick={() => props.onRemove(props.index)}><MdClose/> </Remove>
        <CodeMirror
            onChange={code => props.onChange(props.index, code)}
            value={props.code}
            theme={theme}
            autoFocus
            placeholder={props.index == 0 ? "try '10 m + 2 m'" : undefined}
            extensions={[hotkeys(props.addCellAction)]}
            basicSetup={{
                lineNumbers: props.code.match("\n") != null,
            }}
        />
        {props.code && <Result>
            <AnswerText>Answer:</AnswerText>
            <pre><ResultData dangerouslySetInnerHTML={{__html: converter.toHtml(props.output)}} /></pre>
        </Result>}
    </Wrapper>
});

export default Cell;