import React from 'react';
import { Button, Typography, Popconfirm } from '@arco-design/web-react';
import PermissionWrapper from '@/components/PermissionWrapper';
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
      title: t['searchTable.columns.id'],
      dataIndex: 'id',
      sorter: true,
      render: (value) => <Text copyable>{value}</Text>,
    },
    {
      title: t['searchTable.columns.prize_name'],
      dataIndex: 'prize_name',
      ellipsis: true,
    },
    {
      title: t['searchTable.columns.level'],
      dataIndex: 'level',
      ellipsis: true,
    },
    {
      title: t['searchTable.columns.level_name'],
      dataIndex: 'level_name',
      ellipsis: true,
    },
    {
      title: t['searchTable.columns.probability'],
      dataIndex: 'probability',
      ellipsis: true,
    },
    {
      title: t['searchTable.columns.remaining_quantity'],
      dataIndex: 'remaining_quantity',
      ellipsis: true,
    },
    {
      title: t['searchTable.columns.status'],
      dataIndex: 'status',
      render: (value) =>
        value ? t['searchForm.enable'] : t['searchForm.disable'],
    },
    {
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
      title: t['searchTable.columns.update_time'],
      dataIndex: 'update_time',
      sorter: true,
      render: (x) => {
        if (x != undefined) {
          return dayjs(x).format('YYYY-MM-DD HH:mm:ss');
        }
        return x;
      },
    },
    {
      title: t['searchTable.columns.prize_desc'],
      dataIndex: 'prize_desc',
      ellipsis: true,
    },
    {
      title: t['searchTable.columns.operations'],
      dataIndex: 'operations',
      headerCellStyle: { paddingLeft: '15px' },
      render: (_, record) => [
        <PermissionWrapper
          key={'view'}
          requiredPermissions={[
            { resource: 'live_prize_pool_item_manager', actions: ['api_live_prize_pool_item_info'] },
          ]}
        >
          <Button
            type="text"
            size="small"
            onClick={() => callback(record, 'view')}
          >
            {t['searchTable.columns.operations.view']}
          </Button>
        </PermissionWrapper>,
        <PermissionWrapper
          key={'update'}
          requiredPermissions={[
            { resource: 'live_prize_pool_manager', actions: ['api_live_prize_pool_item_update'] },
          ]}
        >
          <Button
            type="text"
            size="small"
            onClick={() => callback(record, 'update')}
          >
            {t['searchTable.columns.operations.update']}
          </Button>
        </PermissionWrapper>,
      ],
    },
  ];
}
