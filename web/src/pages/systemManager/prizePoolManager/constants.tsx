import React from 'react';
import { Button, Typography, Popconfirm } from '@arco-design/web-react';
import PermissionWrapper from '@/components/PermissionWrapper';
import dayjs from 'dayjs';

const { Text } = Typography;

export const DefaultSorter = {
  field: 'id',
  direction: 'create_time',
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
      title: t['searchTable.columns.pool_name'],
      dataIndex: 'pool_name',
    },
    // {
    //   title: t['searchTable.columns.pool_type'],
    //   dataIndex: 'pool_type',
    // },
    {
      title: t['searchTable.columns.share_pool'],
      dataIndex: 'share_pool',
      render: (value) =>
        value ? t['searchTable.columns.yes'] : t['searchTable.columns.no'],
    },
    // {
    //   title: t['searchTable.columns.strategy'],
    //   dataIndex: 'strategy',
    // },
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
      title: t['searchTable.columns.pool_desc'],
      dataIndex: 'pool_desc',
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
            {
              resource: 'prize_pool_manager',
              actions: ['api_prize_pool_info'],
            },
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
            {
              resource: 'prize_pool_manager',
              actions: ['api_prize_pool_update'],
            },
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
        <PermissionWrapper
          key={'update'}
          requiredPermissions={[
            {
              resource: 'prize_pool_manager',
              actions: ['api_prize_pool_update'],
            },
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
        <PermissionWrapper
          key={'item_manager'}
          requiredPermissions={[{ resource: 'prize_pool_item_manager' }]}
        >
          <Button
            type="text"
            size="small"
            onClick={() => callback(record, 'item_manager')}
          >
            {t['searchTable.columns.operations.item_manager']}
          </Button>
        </PermissionWrapper>,
        <PermissionWrapper
          key={'delete'}
          requiredPermissions={[
            {
              resource: 'prize_pool_manager',
              actions: ['api_prize_pool_delete'],
            },
          ]}
        >
          <Popconfirm
            focusLock
            title={t['option.delete.confirm.title']}
            onOk={() => {
              callback(record, 'delete');
            }}
          >
            <Button status="danger" type="text" size="small">
              {t['searchTable.columns.operations.delete']}
            </Button>
          </Popconfirm>
        </PermissionWrapper>,
      ],
    },
  ];
}
