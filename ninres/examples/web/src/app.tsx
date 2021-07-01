import React, { FC, useState } from 'react';
import ReactJson from 'react-json-view';

import { Button, Card, Link, Page, Spacer, Text } from '@geist-ui/react';

import {
  BNTX,
  Bfres,
  NinResFileExt,
  SfatNode,
  Texture
} from '../../../pkg/ninres';

import { parseFile } from './smmdb';
import { Header } from './header';

export const App: FC = () => {
  const [ninresFiles, setNinresFiles] = useState<NinResFileExt | null>(null);
  const [loading, setLoading] = useState(false);
  let upload: HTMLInputElement | null = null;

  const handleSelect = async (event: React.ChangeEvent<HTMLInputElement>) => {
    if (!event.target.files) return;
    const file = event.target.files[0];
    if (!file) return;
    setLoading(true);
    try {
      const files = await parseFile(file);
      setNinresFiles(files);
    } catch (err) {
      console.error(err);
    }
    setLoading(false);
  };

  const getImageFromBinary = (data: Uint8Array) => {
    const blob = new Blob([data], { type: 'image/png' });
    return URL.createObjectURL(blob);
  };

  return (
    <>
      <Page>
        <Header />
        <Text>
          Please select a resource file (SARC/BFRES). Currently only Super Mario
          Maker 2 resources have tested.
        </Text>
        <Button
          type="success-light"
          onClick={() => {
            if (upload) {
              upload.click();
            }
          }}
          loading={loading}
        >
          Select
        </Button>
        <input
          id="myInput"
          type="file"
          accept=".bfres,.sarc,.pack"
          ref={ref => (upload = ref)}
          style={{ display: 'none' }}
          onChange={handleSelect}
        />
        <Spacer y={2} />
        {ninresFiles &&
          ninresFiles
            .getSarc()
            ?.intoSfatNodes()
            .map((node: SfatNode) => (
              <Card key={node.hash} hoverable shadow>
                <Text h4>{node.getPath()}</Text>
              </Card>
            ))}
        {/* {ninresFiles &&
          ninresFiles
            .getBntxFiles()
            .reduce((arr: JSX.Element[], bntx: BNTX) => {
              console.log('FILES', bntx);
              bntx.getTextures().forEach((texture: Texture) => {
                console.log('texture.getName()', texture.getName());
                arr.push(
                  <div key={texture.getName()}>
                    <span>{texture.getName()}</span>
                    <img
                      style={{ maxWidth: '640px', maxHeight: '360px' }}
                      src={getImageFromBinary(texture.asPng(0, 0)!)}
                    />
                  </div>
                );
              });
              return arr;
              // return (
              //   <div key={i}></div>
              //   // <Card key={course.course.header.creation_id} hoverable shadow>
              //   //   <Text h3>{course.course.header.title}</Text>
              //   //   {course.thumb && (
              //   //     <img
              //   //       style={{ maxWidth: '640px', maxHeight: '360px' }}
              //   //       src={getImageFromBinary(course.thumb.jpeg)}
              //   //     />
              //   //   )}
              //   //   <ReactJson src={course.course} collapsed={1} />
              //   // </Card>
              // );
            }, [])} */}
      </Page>
    </>
  );
};
