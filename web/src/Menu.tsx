import styled from "styled-components";
import { FaBook, FaGithub } from "react-icons/fa";
import Button from "./Button";

const Container = styled.div`
    display: flex;
    width: 100%;
    box-sizing: border-box;
    padding: 1rem;
    justify-content: flex-end;
    align-items: center;  
`;



const Buttons = styled.div`
  display: flex;
  gap: 1rem;
`;

type MenuProps = {
  toggleHelp: () => void;
  helpOpen: boolean;
};

export default function Menu({ toggleHelp, helpOpen }: MenuProps) {
  return (
    <Container className="Menu">
      <Buttons>
        <Button
          onClick={() => location.href = "https://github.com/adelhult/hypatia"}
          title="Github"
          icon={<FaGithub />}
        />
        <Button
          onClick={toggleHelp}
          active={helpOpen}
          title="Help"
          icon={<FaBook />}
        />
      </Buttons>
    </Container>
  );
}
