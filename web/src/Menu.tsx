import styled from "styled-components";
import { FaBook, FaGithub } from "react-icons/fa";
import Button from "./Button";

const Container = styled.div`
    display: flex;
    width: 100%;
    box-sizing: border-box;
    padding: 1rem;
    justify-content: space-between;
    align-items: center;  
`;

const LogoContainer = styled.div`
  display: flex;
  align-items: center;  
`;

const Logo = styled.img`
    width: 6rem;
    margin-right: 0.5rem;
`;

const LogoText = styled.h1`
    font-size: 1.6rem;
    font-weight: normal;
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
      <LogoContainer>
        <Logo src="logo.png"></Logo>
        <LogoText>
          <strong>Hypatia</strong>
          <br />Calculator
        </LogoText>
      </LogoContainer>
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
