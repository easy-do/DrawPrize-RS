import React from 'react';
import { Button, Typography, Badge, Popconfirm } from '@arco-design/web-react';
import dayjs from 'dayjs';
import PermissionWrapper from '@/components/PermissionWrapper';

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
      render: (value) => <Text copyable>{value}</Text>,
    },
    {
      title: t['searchTable.columns.user_name'],
      dataIndex: 'user_name',
    },
    {
      title: t['searchTable.columns.nick_name'],
      dataIndex: 'nick_name',
    },
    {
      title: t['searchTable.columns.status'],
      dataIndex: 'status',
      render: (x) => {
        if (x != undefined && x) {
          return (
            <Badge status="success" text={t['searchForm.all.enable']}></Badge>
          );
        }
        if (x != undefined && !x) {
          return (
            <Badge status="error" text={t['searchForm.all.disable']}></Badge>
          );
        }
        return '-';
      },
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
      title: t['searchTable.columns.last_login_time'],
      dataIndex: 'last_login_time',
      sorter: true,
      render: (x) => {
        if (x != undefined) {
          return dayjs(x).format('YYYY-MM-DD HH:mm:ss');
        }
        return x;
      },
    },
    {
      title: t['searchTable.columns.operations'],
      dataIndex: 'operations',
      headerCellStyle: { paddingLeft: '15px' },
      render: (_, record) => [
        <PermissionWrapper
          key={'view'}
          requiredPermissions={[
            { resource: 'user_manager', actions: ['api_user_info'] },
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
            { resource: 'user_manager', actions: ['api_user_update'] },
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
          key={'set_role'}
          requiredPermissions={[
            { resource: 'user_manager', actions: ['api_user_set_role'] },
          ]}
        >
          <Button
            type="text"
            size="small"
            onClick={() => callback(record, 'set_role')}
          >
            {t['searchTable.columns.operations.setrole']}
          </Button>
        </PermissionWrapper>,
        <PermissionWrapper
          key={'reset_password'}
          requiredPermissions={[
            { resource: 'user_manager', actions: ['api_user_reset_password'] },
          ]}
        >
          <Button
            type="text"
            size="small"
            onClick={() => callback(record, 'reset_password')}
          >
            {t['searchTable.columns.operations.resetpassword']}
          </Button>
        </PermissionWrapper>,
        <PermissionWrapper
          key={'delete'}
          requiredPermissions={[
            { resource: 'user_manager', actions: ['api_user_delete'] },
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
