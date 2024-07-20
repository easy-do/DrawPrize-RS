import React from 'react';
import { Button, Typography } from '@arco-design/web-react';
import dayjs from 'dayjs';

const { Text } = Typography;

export const DefaultSorter = {
  field: 'create_time',
  direction: 'desc',
};

export function getColumns(
  t: any,
  callback: (record: Record<string, any>, type: string) => Promise<void>
) {
  return [
    {
      width: '100px',
      title: t['searchTable.columns.id'],
      dataIndex: 'id',
      sorter: true,
      render: (value) => <Text copyable>{value}</Text>,
    },
    // {
    //   title: t['searchTable.columns.live_id'],
    //   dataIndex: 'live_id',
    //   ellipsis: true,
    // },
    {
      width: '100px',
      title: t['searchTable.columns.action'],
      dataIndex: 'action',
      ellipsis: true,
    },
    {
      width: '200px',
      title: t['searchTable.columns.create_time'],
      dataIndex: 'create_time',
      sorter: true,
      render: (x) => {
        if (x != undefined) {
          return dayjs(x).format('YYYY-MM-DD HH:mm:ss');
        }
        return x;
      },
    },
    {
      title: t['searchTable.columns.prize_ids'],
      dataIndex: 'prize_ids',
      ellipsis: true,
      render: (value) => value? value.split("|").map((item)=>{
        const a =  item.split(',');
        return a[1] + "x1";
       }).join(",") : ''
    },
    // {
    //   title: t['searchTable.columns.operations'],
    //   dataIndex: 'operations',
    //   headerCellStyle: { paddingLeft: '15px' },
    //   render: (_, record) => [
    //       <Button
    //         key={'info'}
    //         type="text"
    //         size="small"
    //         onClick={() => callback(record, 'view')}
    //       >
    //         {t['searchTable.columns.operations.view']}
    //       </Button>
    //   ],
    // },
  ];
}
