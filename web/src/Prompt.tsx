import styled from 'styled-components';
import { MdClose } from "react-icons/md";
import { useState } from "react";
import { motion, AnimatePresence } from 'framer-motion';

const Box = styled(motion.div)`
    position: relative;
    display: flex;
    flex-direction: column;
    border-radius: 0.2rem;
    align-items: flex-start;
    background: #6B8F71;
    color: white;
    padding: 1rem;
    box-sizing: border-box;
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
    opacity: 1;
    cursor: pointer;
    transition: opacity 0.2s;

    &:hover {
        opacity: 1;
    }
`;

const Action = styled.button`
    background: none;
    border: none;
    text-decoration: underline;
    color: inherit;
    font-weight: normal;
    font-family: inherit;
    padding:0;
    font-size: 1rem;
    margin-top: 0.7rem;
    cursor: pointer;
    transition: all 0.2s;

    &:hover {
        background:rgba(1,1,1, 0.1);
    }
`;

const Title = styled.span`
    font-weight: bold;
    font-size: 1.2rem;
    margin-bottom: 0.2rem;
`

interface PromptProps {
    children: React.ReactNode;
    action: string;
    title?: string;
    show: boolean;
    handleAction: () => void;
}

export default function Prompt(props: PromptProps) {
    const [show, setShow] = useState(true);

    return <AnimatePresence>
        {show && props.show && <Box
            initial={{ x: -50, opacity: 0 }}
            animate={{ x: 0, opacity: 1 }}
            exit={{x: 50, opacity: 0}}
            className="Prompt"
        >
            <Remove
                title="Close prompt"
                onClick={() => setShow(false)}
            >
                <MdClose style={{color:"white"}}/>
            </Remove>
            {props.title && <Title>{props.title}</Title>}
            {props.children}
            <Action onClick={props.handleAction}>
                {props.action}
            </Action>
        </Box>
    }
    </AnimatePresence>
}