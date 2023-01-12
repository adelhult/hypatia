import styled from "styled-components";

const LogoContainer = styled.div`
  display: flex;
  align-items: center;
  justify-content: center;
  margin-top: 4rem;
  margin-bottom: 1rem;
`;

const Image = styled.img`
    width: 6rem;
    margin-right: 0.5rem;
`;

const LogoText = styled.div`
    font-size: 1.6rem;
    font-weight: normal;
`;

export default function Logo() {
    return <LogoContainer>
        <Image src="logo.png" />
        <LogoText>
            <strong>Hypatia</strong>
            <br />Calculator
        </LogoText>
    </LogoContainer>
}
