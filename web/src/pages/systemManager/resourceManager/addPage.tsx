import React, { useContext, useEffect, useRef } from 'react';
import {
  Form,
  FormInstance,
  Input,
  Modal,
  Notification,
  Select,
} from '@arco-design/web-react';
import locale from './locale';
import useLocale from '@/utils/useLocale';
import { GlobalContext } from '@/context';
import { API_HTTP_METHOD } from './constants';
import { getResourceList, addResource } from '@/api/resource';
import FormItem from '@arco-design/web-react/es/Form/form-item';

function AddPage(props: { visible; setVisible; callback: () => void }) {
  const formRef = useRef<FormInstance>();

  const { lang } = useContext(GlobalContext);

  const t = useLocale(locale);

  const [loading, setLoading] = React.useState(false);

  const [resourceTree, setResourceTree] = React.useState([]);

  useEffect(() => {
    getResourceList().then((res) => {
      const { success, data } = res.data;
      if (success) {
        setResourceTree(
          data.map((item) => {
            return {
              label: item.resource_name,
              value: item.id,
            };
          })
        );
      }
    });
  }, []);

  const handleSubmit = () => {
    formRef.current.validate().then((values) => {
      setLoading(true);
      values.resource_root = values.resource_root == 'true';
      values.resource_action = values.resource_action == 'true';
      values.status = values.status == 'true';
      values.order_number = Number(values.order_number);
      addResource(values)
        .then((res) => {
          const { success, message } = res.data;
          if (success) {
            Notification.success({ content: message, duration: 1000 });
            props.setVisible(false);
            props.callback();
          }
        })
        .finally(() => {
          setLoading(false);
        });
    });
  };

  return (
    <Modal
      title={t['searchTable.operations.add']}
      visible={props.visible}
      onOk={() => {
        handleSubmit();
      }}
      onCancel={() => {
        props.setVisible(false);
      }}
      autoFocus={false}
      focusLock={true}
      confirmLoading={loading}
    >
      <Form
        ref={formRef}
        style={{ width: '95%', marginTop: '6px' }}
        labelCol={{ span: lang === 'en-US' ? 7 : 6 }}
        wrapperCol={{ span: lang === 'en-US' ? 17 : 18 }}
      >
        <FormItem
          initialValue={0}
          label={t['searchTable.columns.parent_id']}
          field={'parent_id'}
        >
          <Select
            showSearch
            placeholder={t['searchForm.placeholder']}
            options={resourceTree}
            allowClear
            loading={loading}
            filterOption={(inputValue, option) =>
              option.props.children.indexOf(inputValue) >= 0
            }
          />
        </FormItem>

        <FormItem
          required
          label={t['searchTable.columns.resource_name']}
          field={'resource_name'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.resource_code']}
          field={'resource_code'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.resource_type']}
          field={'resource_type'}
        >
          <Select
            placeholder={t['searchForm.placeholder']}
            options={[
              {
                label: '菜单',
                value: 1,
              },
              {
                label: '功能',
                value: 2,
              },
            ]}
            allowClear
            loading={loading}
          />
        </FormItem>
        <FormItem
          required
          initialValue={'false'}
          label={t['searchTable.columns.resource_root']}
          field={'resource_root'}
        >
          <Select
            placeholder={t['searchForm.placeholder']}
            options={[
              {
                label: t['searchTable.columns.yes'],
                value: 'true',
              },
              {
                label: t['searchTable.columns.no'],
                value: 'false',
              },
            ]}
            allowClear
          />
        </FormItem>
        <FormItem
          required
          initialValue={'false'}
          label={t['searchTable.columns.resource_action']}
          field={'resource_action'}
        >
          <Select
            placeholder={t['searchForm.placeholder']}
            options={[
              {
                label: t['searchTable.columns.yes'],
                value: 'true',
              },
              {
                label: t['searchTable.columns.no'],
                value: 'false',
              },
            ]}
            allowClear
          />
        </FormItem>
        <FormItem
          required
          initialValue={0}
          label={t['searchTable.columns.order_number']}
          field={'order_number'}
          itemType="number"
        >
          <Input
            type="number"
            min={0}
            placeholder={t['searchForm.placeholder']}
            allowClear
          />
        </FormItem>
        <FormItem
          required
          initialValue={'true'}
          label={t['searchTable.columns.status']}
          field={'status'}
        >
          <Select
            placeholder={t['searchForm.placeholder']}
            options={[
              {
                label: t['searchForm.enable'],
                value: 'true',
              },
              {
                label: t['searchForm.disable'],
                value: 'false',
              },
            ]}
            allowClear
          />
        </FormItem>
        <FormItem label={t['searchTable.columns.url']} field={'url'}>
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem label={t['searchTable.columns.icon']} field={'icon'}>
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem label={t['searchTable.columns.api_path']} field={'api_path'}>
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem
          label={t['searchTable.columns.api_http_method']}
          field={'api_http_method'}
        >
          <Select
            placeholder={t['searchForm.placeholder']}
            options={API_HTTP_METHOD.map((item) => {
              return {
                label: item,
                value: item,
              };
            })}
            allowClear
          />
        </FormItem>
        <FormItem
          label={t['searchTable.columns.api_path_regex']}
          field={'api_path_regex'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem label={t['searchTable.columns.role']} field={'role'}>
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem
          label={t['searchTable.columns.resource_desc']}
          field={'resource_desc'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
      </Form>
    </Modal>
  );
}

export default AddPage;
