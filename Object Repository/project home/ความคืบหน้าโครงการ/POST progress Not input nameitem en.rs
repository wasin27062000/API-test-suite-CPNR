<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST progress Not input nameitem en</name>
   <tag></tag>
   <elementGuidId>054de51e-7de6-4c84-add7-580dd0158c3b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: null,\n  \&quot;name\&quot;: {\n    \&quot;th\&quot;: \&quot;ความคืบหน้าครั้งที่ 1\&quot;,\n    \&quot;en\&quot;: \&quot;Progress No.1\&quot;\n  },\n  \&quot;items\&quot;: [\n    {\n      \&quot;id\&quot;: null,\n      \&quot;percentage\&quot;: 2,\n      \&quot;selected\&quot;: true,\n      \&quot;name\&quot;: {\n        \&quot;th\&quot;: \&quot;วางโครงสร้าง\&quot;,\n        \&quot;en\&quot;: null\n      },\n      \&quot;description\&quot;: {\n        \&quot;th\&quot;: \&quot;วางโครงสร้าง\&quot;,\n        \&quot;en\&quot;: \&quot;วางโครงสร้าง\&quot;\n      }\n    }\n  ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>4c8db673-6962-4496-9cf9-e6f3cfa2a5b8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>63785644-736a-4cd1-a749-c52e594e6a8f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://45.150.131.146:7090/api-backoffice/project/${id_project}/progress</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>60f41412-46fc-406c-86bf-cbb7525c4841</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_project</defaultValue>
      <description></description>
      <id>f2b377f8-b3d3-4d7a-939d-05a21068faeb</id>
      <masked>false</masked>
      <name>id_project</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
