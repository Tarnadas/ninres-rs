import React, { FC, useState } from 'react';

import { Button, Page, Spacer, Text } from '@geist-ui/react';

import { NinResFileExt } from '../../../pkg/ninres';

import { parseFile } from './smmdb';
import { Header } from './header';
import { Ninres } from './ninres';

export const App: FC = () => {
  const [ninresFile, setNinresFile] = useState<NinResFileExt | null>(null);
  const [loading, setLoading] = useState(false);
  let upload: HTMLInputElement | null = null;

  const handleSelect = async (event: React.ChangeEvent<HTMLInputElement>) => {
    if (!event.target.files) return;
    const file = event.target.files[0];
    if (!file) return;
    setLoading(true);
    try {
      const files = await parseFile(file);
      setNinresFile(files);
    } catch (err) {
      console.error(err);
    }
    setLoading(false);
  };

  return (
    <>
      <Page>
        <Header />
        <Text>
          Please select a resource file (SARC/BFRES). Currently only Super Mario
          Maker 2 resources have been tested.
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

        {ninresFile && <Ninres ninres={ninresFile} />}
      </Page>
    </>
  );
};
