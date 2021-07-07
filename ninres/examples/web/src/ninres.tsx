import React, { FC, useState } from 'react';

import { Button, Card, Grid, Text } from '@geist-ui/react';

import { BNTX, NinResFileExt, SfatNode, Texture } from '../../../pkg/ninres';

import { parseData } from './smmdb';

export const Ninres: FC<{ ninres: NinResFileExt }> = ({ ninres }) => {
  const [ninresFileMap, setNinresFileMap] = useState<{
    [key: number]: NinResFileExt;
  }>({});
  const [loading, setLoading] = useState(false);

  const handleExtract = (node: SfatNode) => async () => {
    if (loading) return;
    setLoading(true);
    await new Promise(resolve => setTimeout(resolve));
    try {
      const { hash } = node;
      const files = await parseData(node.intoData());
      setNinresFileMap({
        [hash]: files,
        ...ninresFileMap
      });
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
      <Grid.Container gap={2} justify="center">
        {ninres
          .getSarc()
          ?.intoSfatNodes()
          .map((node: SfatNode) => {
            const nextFiles = ninresFileMap[node.hash];
            const path = node.getPath();
            const canExtract =
              !nextFiles &&
              path &&
              (path.includes('.zs') || path.includes('.bfres'));
            return (
              <Grid key={node.hash} xs={24}>
                <Card hoverable shadow>
                  <Text h4 style={{ wordBreak: 'break-word' }}>
                    {path}
                  </Text>

                  {canExtract ? (
                    <Button
                      type="success"
                      ghost
                      onClick={handleExtract(node)}
                      loading={loading}
                    >
                      Extract
                    </Button>
                  ) : (
                    nextFiles && <Ninres ninres={nextFiles} />
                  )}
                </Card>
              </Grid>
            );
          })}

        {ninres
          .getBfres()
          ?.intoBntxFiles()
          .reduce((arr: JSX.Element[], bntx: BNTX) => {
            bntx.getTextures().forEach((texture: Texture) => {
              const png = texture.asPng(0, 0);
              arr.push(
                <Grid key={texture.getName()} xs={8} sm={6} md={4} lg={3}>
                  <Card hoverable shadow>
                    <Text h6 style={{ wordBreak: 'break-word' }}>
                      {texture.getName()}
                    </Text>

                    {png && (
                      <img
                        style={{ maxWidth: '640px', maxHeight: '360px' }}
                        src={getImageFromBinary(png)}
                      />
                    )}
                  </Card>
                </Grid>
              );
            });
            return arr;
          }, [])}
      </Grid.Container>
    </>
  );
};
