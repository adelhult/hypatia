import styled from "styled-components";

const LogoContainer = styled.div`
  display: flex;
  align-items: center;
  justify-content: center;
`;

const Image = styled.img`
    width: 6rem;
    margin-right: 0.5rem;
`;

const LogoText = styled.div`
    font-size: 1.6rem;
    font-weight: normal;
`;

export default function Logo({ className }: { className: string }) {
    return <LogoContainer className={className}>
        <Image src="logo.png" />
        <LogoText>
            <strong>Hypatia</strong>
            <br />Calculator
        </LogoText>
    </LogoContainer>
}
