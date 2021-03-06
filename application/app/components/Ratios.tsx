import React from 'react';
import styled from 'styled-components';
// import path from 'path';
// import { remote } from 'electron';
// import { Demo, DemoData } from './Tutorial';
// import { tutorial_list, album_list } from './tutorial_list';
import { RatioChart } from './RatioChart';

const RSpace = styled.div`
  position: absolute;
  top: 10;
  right: 0;
  display: flex;
  flex-direction: column;
  font-family: 'Courier New', Courier, monospace;
  font-size: 1.1em;
  margin-right: 1em;
  text-align: center;
  border: 5px ridge goldenrod;
`;

// const MagicButton = styled.img`
// width: 70px;
// height: 70px;
// border-top: 5px ridge goldenrod;
// opacity: 0.7;
// background-color: red;
// :hover {
// opacity: 1;
// }
// `;

// const MagicButtonSmall = styled.img`
// width: 40px;
// height: 40px;
// border-top: 5px ridge goldenrod;
// opacity: 0.7;
// background-color: red;
// :hover {
// opacity: 1;
// }
// `;

export const Ratios = (props: { width: number }): React.ReactElement | null => {
  // const assetsPath = remote.app.isPackaged
  // ? path.join(process.resourcesPath, 'extraResources/assets')
  // : '../extraResources/assets';
  // const [showTutorial, setShowTutorial] = React.useState(false);
  // const [showDemo, setShowDemo] = React.useState(false);

  // const showTutorialModal = (b: boolean) => {
  // setShowTutorial(b);
  // };

  // const showDemoModal = (b: boolean) => {
  // setShowDemo(b);
  // };

  // const tutorialData: DemoData = {
  // title: 'Cool Tutorials',
  // setShow: showTutorialModal,
  // show: showTutorial,
  // data: tutorial_list,
  // folder: 'tutorial',
  // };

  // const demoData: DemoData = {
  // title: 'Cool Demos',
  // setShow: showDemoModal,
  // show: showDemo,
  // data: album_list,
  // folder: 'demo',
  // };

  if (props.width > 1000) {
    return (
      <div>
        <RSpace id="ratios">
          <RatioChart />
        </RSpace>
      </div>
    );
  } else if (props.width > 700) {
    return <div />;
  } else {
    return <div />;
  }
};
